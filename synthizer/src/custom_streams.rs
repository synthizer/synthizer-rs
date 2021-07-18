//! Implement the infrastructure for custom streams.
//!
//! We actually get to do this on top of `Read` and `Seek` directly.
use crate::internal_prelude::*;
use std::borrow::Borrow;
use std::ffi::{c_void, CString};
use std::io::{Read, Seek};
use std::os::raw::{c_char, c_int, c_longlong, c_ulonglong};
use std::slice::from_raw_parts_mut;

/// A trait which custom streams must implement in order to support closing.
///
/// Rust's stdlib has no concept of closing in it, but simply dropping values
/// leads to panics.  This trait is therefore required to implement closing.
pub trait CloseStream {
    fn close(&mut self) -> std::result::Result<(), Box<dyn std::fmt::Display>>;
}

/// Marker trait for types which implement non-seekable streams.
///
/// A blanket impl is provided for anything implementing the supertraits.
pub trait Stream: Read + CloseStream + Send + 'static {}

impl<T: Read + CloseStream + Send + 'static> Stream for T {}

/// A [Stream], but one which also implements [Seek].
///
/// blanket impls are provided for anything implementing [Stream] and [Seek]
pub trait SeekableStream: Stream + Seek {}
impl<T: Stream + Seek> SeekableStream for T {}

struct CustomStreamData<T> {
    err_msg: CString,
    stream: T,
}

impl<T> CustomStreamData<T> {
    fn conv_err(&mut self, data: &dyn std::fmt::Display) -> *const c_char {
        let str = format!("{}", data);
        let cstr = CString::new(str).expect("Display impls must produce valid C strings");
        self.err_msg = cstr;
        self.err_msg.as_ptr()
    }
}

extern "C" fn stream_read_cb<T: Read>(
    read: *mut c_ulonglong,
    requested: c_ulonglong,
    destination: *mut c_char,
    userdata: *mut c_void,
    err_msg: *mut *const c_char,
) -> c_int {
    let data = unsafe { &mut *(userdata as *mut CustomStreamData<T>) };

    let dest = unsafe { from_raw_parts_mut(destination as *mut u8, requested as usize) };
    match data.stream.read(dest) {
        Ok(d) => {
            unsafe { *read = d as c_ulonglong };
            0
        }
        Err(e) => {
            unsafe { *err_msg = data.conv_err(&e) };
            1
        }
    }
}

extern "C" fn seek_cb<T: Seek>(
    pos: c_ulonglong,
    userdata: *mut c_void,
    err_msg: *mut *const c_char,
) -> c_int {
    let data = unsafe { &mut *(userdata as *mut CustomStreamData<T>) };

    match data.stream.seek(std::io::SeekFrom::Start(pos as u64)) {
        Ok(_) => 0,
        Err(e) => {
            unsafe { *err_msg = data.conv_err(&e) };
            1
        }
    }
}

extern "C" fn close_cb<T: CloseStream>(
    userdata: *mut c_void,
    err_msg: *mut *const c_char,
) -> c_int {
    let data = unsafe { &mut *(userdata as *mut CustomStreamData<T>) };

    match data.stream.close() {
        Ok(_) => 0,
        Err(e) => {
            unsafe { *err_msg = data.conv_err(&e) };
            1
        }
    }
}

extern "C" fn destroy_cb<T>(userdata: *mut c_void) {
    // Build a box and immediately drop it.
    unsafe { Box::from_raw(userdata as *mut CustomStreamData<T>) };
}

/// Used as part of [StreamHandle] consumption.
fn drop_cb<T>(ptr: *mut c_void) {
    unsafe {
        Box::<T>::from_raw(ptr as *mut T);
    }
}

fn fillout_read<T: Stream>(dest: &mut syz_CustomStreamDef) {
    dest.read_cb = Some(stream_read_cb::<T>);
    dest.close_cb = Some(close_cb::<T>);
    dest.destroy_cb = Some(destroy_cb::<T>);
    dest.length = -1;
}

fn fillout_seekable<T: SeekableStream>(
    dest: &mut syz_CustomStreamDef,
    val: &mut T,
) -> std::io::Result<()> {
    dest.seek_cb = Some(seek_cb::<T>);

    val.seek(std::io::SeekFrom::End(0))?;
    dest.length = val.stream_position()? as c_longlong;
    val.seek(std::io::SeekFrom::Start(0))?;
    Ok(())
}

fn fillout_userdata<T>(dest: &mut syz_CustomStreamDef, val: T) {
    dest.userdata = Box::into_raw(Box::new(CustomStreamData {
        stream: val,
        err_msg: Default::default(),
    })) as *mut c_void;
}

/// A definition for a custom stream.  This can come from a variety of places
/// and is consumed by e.g. [StreamingGenerator::from_stream_handle], or
/// returned as a result of the callback passed to [register_stream_protocol].
pub struct CustomStreamDef {
    def: syz_CustomStreamDef,
    /// Has this handle been used yet? If not, the drop impl needs to do cleanup
    /// so that the user's failure to consume the value with [CustomStreamDef]
    /// does not leak their value.
    used: bool,
}

impl CustomStreamDef {
    /// Convert a [Read] to a stream.
    pub fn from_reader<T: Stream>(value: T) -> CustomStreamDef {
        let mut ret = CustomStreamDef {
            def: Default::default(),
            used: false,
        };

        fillout_read::<T>(&mut ret.def);
        fillout_userdata(&mut ret.def, value);
        ret
    }

    /// Build a stream from something seekable.
    pub fn from_seekable<T: SeekableStream>(mut value: T) -> std::io::Result<CustomStreamDef> {
        let mut ret = CustomStreamDef {
            def: Default::default(),
            used: false,
        };
        fillout_read::<T>(&mut ret.def);
        fillout_seekable(&mut ret.def, &mut value)?;
        fillout_userdata(&mut ret.def, value);
        Ok(ret)
    }
}

impl Drop for CustomStreamDef {
    fn drop(&mut self) {
        let mut err_msg: *const c_char = std::ptr::null();
        if !self.used {
            unsafe {
                if let Some(cb) = self.def.close_cb {
                    cb(self.def.userdata, &mut err_msg as *mut *const c_char);
                }
                if let Some(cb) = self.def.destroy_cb {
                    cb(self.def.userdata);
                }
            }
        }
    }
}

/// A `StreamHandle` binds Synthizer custom streams, as well as other kinds of
/// streaming functionality.
pub struct StreamHandle {
    handle: syz_Handle,
    // If set, this stream will move the given value into Synthizer userdata for freeing later.
    needs_drop: Option<(std::ptr::NonNull<c_void>, fn(*mut c_void))>,
}

impl StreamHandle {
    pub fn from_stream_def(mut def: CustomStreamDef) -> Result<StreamHandle> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createStreamHandleFromCustomStream(
                &mut h as *mut syz_Handle,
                &mut def.def as *mut syz_CustomStreamDef,
                std::ptr::null_mut(),
                None,
            )
        })?;
        def.used = true;

        Ok(StreamHandle {
            handle: h,
            needs_drop: None,
        })
    }

    /// Create a stream handle which is backed by memory.
    pub fn from_vec(data: Vec<u8>) -> Result<StreamHandle> {
        if data.is_empty() {
            return Err(Error::rust_error("Cannot create streams from empty vecs"));
        };
        let mut h = Default::default();
        check_error(unsafe {
            let ptr = &data[0] as *const u8 as *const i8;
            syz_createStreamHandleFromMemory(
                &mut h as *mut syz_Handle,
                data.len() as u64,
                ptr,
                std::ptr::null_mut(),
                None,
            )
        })?;

        Ok(StreamHandle {
            handle: h,
            needs_drop: Some((
                unsafe {
                    std::ptr::NonNull::new_unchecked(Box::into_raw(Box::new(data)) as *mut c_void)
                },
                drop_cb::<Vec<u8>>,
            )),
        })
    }

    pub fn from_stream_params(protocol: &str, path: &str, param: usize) -> Result<StreamHandle> {
        // The below transmute uses the fact that `usize` is the size of a
        // pointer on all common platforms.
        let mut h = Default::default();
        let protocol_c = std::ffi::CString::new(protocol)
            .map_err(|_| Error::rust_error("Unable to convert protocol to a C string"))?;
        let path_c = std::ffi::CString::new(path)
            .map_err(|_| Error::rust_error("Unable to convert path to a C string"))?;
        let protocol_ptr = protocol_c.as_ptr();
        let path_ptr = path_c.as_ptr();
        check_error(unsafe {
            syz_createStreamHandleFromStreamParams(
                &mut h as *mut syz_Handle,
                protocol_ptr as *const c_char,
                path_ptr as *const c_char,
                std::mem::transmute(param),
                std::ptr::null_mut(),
                None,
            )
        })?;
        Ok(StreamHandle {
            handle: h,
            needs_drop: None,
        })
    }

    pub(crate) fn get_handle(&self) -> syz_Handle {
        self.handle
    }

    fn get_userdata(mut self) -> UserdataBox {
        // Be sure to take here so that Drop doesn't try to double free.
        let ret = if let Some((ud, free_cb)) = self.needs_drop.take() {
            UserdataBox::from_streaming_userdata(ud, free_cb)
        } else {
            UserdataBox::new()
        };
        ret
    }

    /// Wrap getting userdata and also make sure to free the handle once the
    /// closure ends, regardless of if it succeeded.
    // The closure gets the stream handle, as well as the userdata pointer and
    // free callback.
    pub(crate) fn with_userdata<T>(
        mut self,
        mut closure: impl (FnMut(syz_Handle, *mut c_void, extern "C" fn(*mut c_void)) -> Result<T>),
    ) -> Result<T> {
        let sh = self.handle;
        // Take the handle.
        self.handle = 0;
        let ud = self.get_userdata();
        ud.consume(move |ud, cb| closure(sh, ud, cb))
    }
}

impl Drop for StreamHandle {
    fn drop(&mut self) {
        unsafe { syz_handleDecRef(self.handle) };
        if let Some((ud, cb)) = self.needs_drop {
            cb(ud.as_ptr());
        }
    }
}

static mut STREAM_ERR_CONSTANT: *const c_char = std::ptr::null();

extern "C" fn stream_open_callback<
    E,
    T: 'static + Send + Sync + Fn(&str, &str, usize) -> std::result::Result<CustomStreamDef, E>,
>(
    out: *mut syz_CustomStreamDef,
    protocol: *const c_char,
    path: *const c_char,
    param: *mut c_void,
    userdata: *mut c_void,
    err_msg: *mut *const c_char,
) -> c_int {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let cstr = std::ffi::CString::new("Unable to create stream").unwrap();
        unsafe { STREAM_ERR_CONSTANT = cstr.into_raw() };
    });

    let protocol = unsafe { std::ffi::CStr::from_ptr(protocol) };
    let path = unsafe { std::ffi::CStr::from_ptr(path) };
    let protocol = protocol.to_string_lossy();
    let path = path.to_string_lossy();
    let param: usize = unsafe { std::mem::transmute(param) };

    let cb: Box<T> = unsafe { Box::from_raw(userdata as *mut T) };
    let res = cb(protocol.borrow(), path.borrow(), param);
    // Be sure not to drop the callback.
    Box::into_raw(cb);

    match res {
        Ok(mut s) => {
            unsafe { *out = s.def };
            s.used = true;
            0
        }
        Err(_) => {
            unsafe { *err_msg = STREAM_ERR_CONSTANT };
            1
        }
    }
}

/// register a custom protocol.
///
/// The callback here must return a [CustomStreamDef] which represents the
/// custom stream.  Synthizer is also not safely reentrant, and the callback
/// must not call back into Synthizer.
pub fn register_stream_protocol<
    E,
    T: 'static + Send + Sync + Fn(&str, &str, usize) -> std::result::Result<CustomStreamDef, E>,
>(
    protocol: &str,
    callback: T,
) -> Result<()> {
    let protocol_c = std::ffi::CString::new(protocol)
        .map_err(|_| Error::rust_error("Unable to convert protocol to a C string"))?;
    let leaked = Box::into_raw(Box::new(callback));
    let res = check_error(unsafe {
        syz_registerStreamProtocol(
            protocol_c.as_ptr(),
            Some(stream_open_callback::<E, T>),
            leaked as *mut c_void,
        )
    });
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            unsafe { Box::from_raw(leaked) };
            Err(e)
        }
    }
}
