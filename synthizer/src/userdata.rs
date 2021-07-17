//! Provides the internal machinery for hooking up userdata.
use std::any::Any;
use std::ffi::c_void;
use std::sync::{Arc, RwLock};

use crate::internal_prelude::*;

/// The userdata in Synthizer never changes, but the values here can.
pub(crate) struct UserdataBox {
    userdata: RwLock<Option<Arc<dyn Any + Send + Sync>>>,
    /// For streaming/buffer stuff.
    streaming_userdata: Option<(std::ptr::NonNull<c_void>, unsafe fn(*mut c_void))>,
}

extern "C" fn drop_userdata_cb(ptr: *mut c_void) {
    unsafe { Box::from_raw(ptr as *mut UserdataBox) };
}

impl UserdataBox {
    pub(crate) fn new() -> UserdataBox {
        UserdataBox {
            userdata: RwLock::new(None),
            streaming_userdata: None,
        }
    }

    pub(crate) fn from_streaming_userdata(
        ptr: std::ptr::NonNull<c_void>,
        free_cb: fn(*mut c_void),
    ) -> UserdataBox {
        UserdataBox {
            userdata: RwLock::new(None),
            streaming_userdata: Some((ptr, free_cb)),
        }
    }

    pub(crate) fn set_userdata(&self, ud: Option<impl Any + Send + Sync>) {
        let mut guard = self.userdata.write().expect("Mutex poisoned");
        *guard = ud.map(|x| Arc::new(x) as Arc<dyn Any + Send + Sync>);
    }

    pub(crate) fn get_userdata(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        (*self.userdata.read().expect("Mutex poisoned")).clone()
    }

    /// Consume this `UserdataBox`.  if the provided closure succeeds, the
    /// closure has taken ownership of the values. Otherwise, the object is
    /// safely dropped.
    pub(crate) fn consume<T>(
        self,
        mut closure: impl (FnMut(*mut c_void, extern "C" fn(*mut c_void)) -> Result<T>),
    ) -> Result<T> {
        let leaked = Box::into_raw(Box::new(self)) as *mut c_void;
        let res = closure(leaked, drop_userdata_cb);
        if let Err(_) = res {
            unsafe { Box::from_raw(leaked as *mut UserdataBox) };
        }
        res
    }
}

impl Default for UserdataBox {
    fn default() -> UserdataBox {
        UserdataBox::new()
    }
}

impl Drop for UserdataBox {
    fn drop(&mut self) {
        match self.streaming_userdata {
            Some((ptr, cb)) => {
                unsafe { cb(ptr.as_ptr()) };
            }
            _ => {}
        }
    }
}

/// Helper function to make a userdata pair for the constructor.
pub(crate) fn wrap_constructor<T>(
    closure: impl (FnMut(*mut c_void, extern "C" fn(*mut c_void)) -> Result<T>),
) -> Result<T> {
    UserdataBox::new().consume(closure)
}
