//! Implement the infrastructure for custom streams.
//!
//! We actually get to do this on top of `Read` and `Seek` directly.
use std::ffi::{c_void, CString};
use std::io::{Read, Seek};
use std::os::raw::{c_char, c_int, c_longlong, c_ulonglong};
use std::slice::from_raw_parts_mut;

use synthizer_sys::*;

use crate::errors::*;
use crate::*;

/// A trait which custom streams must implement in order to support closing.
///
/// Rust's stdlib has no concept of closing in it, but simply dropping values leads to panics.  This trait is therefore required to implement closing.
pub trait CloseStream {
    fn close(&self) -> std::result::Result<(), Box<dyn std::fmt::Display>>;
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
extern "C" fn drop_cb<T>(ptr: *mut c_void) {
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
    dest.userdata = Box::into_raw(Box::new(val)) as *mut c_void;
}

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
    pub fn from_seek<T: SeekableStream>(mut value: T) -> std::io::Result<CustomStreamDef> {
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
                self.def
                    .close_cb
                    .map(|x| x(self.def.userdata, &mut err_msg as *mut *const c_char));
                self.def.destroy_cb.map(|x| x(self.def.userdata));
            }
        }
    }
}

/// A `StreamHandle` binds Synthizer custom streams, as well as other kinds of streaming functionality.
pub struct StreamHandle {
    handle: syz_Handle,
    // If set, this stream will move the given value into Synthizer userdata for freeing later.
    needs_drop: Option<(std::ptr::NonNull<c_void>, extern "C" fn(*mut c_void))>,
    used: bool,
}

impl StreamHandle {
    pub fn from_custom(mut def: CustomStreamDef) -> Result<StreamHandle> {
        let mut h = Default::default();
        check_error(unsafe {
            syz_createStreamHandleFromCustomStream(
                &mut h as *mut syz_Handle,
                &mut def.def as *mut syz_CustomStreamDef,
            )
        })?;
        def.used = true;

        Ok(StreamHandle {
            handle: h,
            needs_drop: None,
            used: false,
        })
    }

    /// Create a stream handle which is backed by memory.
    pub fn from_vec(data: Vec<u8>) -> Result<StreamHandle> {
        if data.is_empty() {
            return Err(Error::rust_error("Cannot create streams from empty vecs"));
        };
        let mut h = Default::default();
        check_error(unsafe {
            syz_createStreamHandleFromMemory(
                &mut h as *mut syz_Handle,
                data.len() as u64,
                std::mem::transmute(&data[0] as *const u8),
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
            used: false,
        })
    }

    pub(crate) fn get_handle(&self) -> syz_Handle {
        self.handle
    }

    /// Consume the handle, linking it to the other handle via Synthizer's userdata support as necessary.
    pub(crate) fn link(mut self, handle: &Handle) -> Result<()> {
        if let Some((ud, free_cb)) = self.needs_drop.take() {
            check_error(unsafe {
                syz_setUserdata(handle.to_syz_handle(), ud.as_ptr(), Some(free_cb))
            })?;
        }
        self.used = true;
        Ok(())
    }
}

impl Drop for StreamHandle {
    fn drop(&mut self) {
        if self.used {
            return;
        }

        unsafe { syz_handleDecRef(self.handle) };
        if let Some((ud, cb)) = self.needs_drop {
            cb(ud.as_ptr());
        }
    }
}
