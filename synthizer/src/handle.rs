use synthizer_sys::*;

use crate::casting::*;
use crate::errors::*;
use crate::*;

pub struct Handle(pub(crate) syz_Handle);

impl Handle {
    pub fn handle(&self) -> &Handle {
        &self
    }

    pub fn into_handle(self) -> Handle {
        self
    }

    /// Get the object's type if possible.  THis function will fail if Synthizer is not initialized.
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
    /// because that's public: people should be using the bindings, not converting
    /// handles directly.
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
                self.0 .0
            }
        }

        impl HandleRef for $t {
            fn handle_ref(&self) -> &Handle {
                &self.0
            }
        }
    };
}
