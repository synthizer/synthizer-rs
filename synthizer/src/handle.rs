use crate::internal_prelude::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Handle(syz_Handle);

impl Handle {
    pub fn new(h: syz_Handle) -> Handle {
        Handle(h)
    }

    pub fn handle(&self) -> &Handle {
        &self
    }

    pub fn into_handle(self) -> Handle {
        self
    }

    /// Get the object's type if possible.  THis function will fail if Synthizer
    /// is not initialized.
    pub fn get_type(&self) -> Result<ObjectType> {
        let mut out = 0;
        check_error(unsafe {
            syz_handleGetObjectType(&mut out as *mut i32, self.to_syz_handle())
        })?;
        Ok(unsafe { std::mem::transmute(out) })
    }

    /// Try to cast this object to another object type.  Will return
    /// `Ok(None)` if this is because of a type mismatch, otherwise `Err`.
    /// Clones `self` on success in order to prevent throwing the object
    /// away on error.
    pub fn cast_to<T: CastTarget>(&self) -> Result<Option<T>> {
        T::cast_from(self.handle_ref())
    }

    /// Get this handle's userdata.
    pub fn get_userdata(&self) -> Result<Option<Arc<dyn Any + Send + Sync>>> {
        with_witness(|| {
            let b = self.get_userdata_box()?;
            Ok(unsafe { b.as_mut().expect("Is always non-NULL").get_userdata() }.clone())
        })
    }

    /// Set the userdata for this handle.
    pub fn set_userdata(&self, userdata: Option<impl Any + Send + Sync>) -> Result<()> {
        with_witness(move || {
            let b = self.get_userdata_box()?;
            unsafe {
                b.as_mut()
                    .expect("Pointer is always valid")
                    .set_userdata(userdata)
            };
            Ok(())
        })
    }

    /// Needs to be called inside of [with_witness].
    fn get_userdata_box(&self) -> Result<*mut UserdataBox> {
        let mut ud: *mut std::ffi::c_void = std::ptr::null_mut();
        check_error(unsafe {
            syz_handleGetUserdata(&mut ud as *mut *mut std::ffi::c_void, self.0)
        })?;
        let ud_box = ud as *mut UserdataBox;
        Ok(ud_box)
    }

    pub fn config_delete_behavior(&self, config: &DeleteBehaviorConfig) -> Result<()> {
        check_error(unsafe {
            syz_configDeleteBehavior(self.0, &config.cfg as *const syz_DeleteBehaviorConfig)
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_handle_internal(h: Handle) -> Handle {
        h
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        check_error(unsafe { syz_handleDecRef(self.0) })
            .expect("Dropping handles should not error");
    }
}

impl Clone for Handle {
    fn clone(&self) -> Handle {
        check_error(unsafe { syz_handleIncRef(self.0) })
            .expect("Incrementing reference counts should never error");
        Handle(self.0)
    }
}

// A weird workaround to keep the `ToHandle` trait private.  I actually don't
// understand why this works, but it does.
mod priv_traits {
    use super::*;

    /// Internal helper trait to convert things to handles.  We don't use `From`
    /// because that's public: people should be using the bindings, not
    /// converting handles directly.
    pub trait ToSyzHandle {
        fn to_syz_handle(&self) -> syz_Handle;
    }

    /// Trait to get a reference to the handle.
    pub trait HandleRef {
        fn handle_ref(&self) -> &Handle;
    }
}

pub(crate) use priv_traits::*;

impl ToSyzHandle for Handle {
    fn to_syz_handle(&self) -> syz_Handle {
        self.0
    }
}

impl HandleRef for Handle {
    fn handle_ref(&self) -> &Handle {
        &self
    }
}

macro_rules! handle_traits {
    ($t: ty) => {
        impl ToSyzHandle for $t {
            fn to_syz_handle(&self) -> syz_Handle {
                self.0.to_syz_handle()
            }
        }

        impl HandleRef for $t {
            fn handle_ref(&self) -> &Handle {
                &self.0
            }
        }
    };
}
