//! Define the "base classes" through macro magic.E.g. `source_common` is the
//! source "base class".

macro_rules! object_common {
    () => {
        pub fn handle(&self) -> &Handle {
            &self.0
        }

        pub fn into_handle(self) -> Handle {
            self.0
        }

        pub fn get_type(&self) -> Result<ObjectType> {
            self.0.get_type()
        }

        /// Try to cast this object to another object type.  Will return
        /// `Ok(None)` if this is because of a type mismatch, otherwise `Err`.
        /// Clones `self` on success in order to prevent throwing the object
        /// away on error.
        pub fn cast_to<T: CastTarget>(&self) -> Result<Option<T>> {
            T::cast_from(self.handle_ref())
        }

        pub fn get_userdata(&self) -> Result<Option<Arc<dyn Any + Send + Sync>>> {
            self.0.get_userdata()
        }

        pub fn set_userdata(&self, userdata: Option<impl Any + Send + Sync>) -> Result<()> {
            self.0.set_userdata(userdata)
        }

        pub fn config_delete_behavior(&self, config: &DeleteBehaviorConfig) -> Result<()> {
            self.0.config_delete_behavior(config)
        }

        /// Internal function to get objects from handles, used in
        /// casting.rs to enable casting from impls behind a macro.  This
        /// can't be pub: converting a handle of the worng type to a
        /// specific object breaks Synthizer, though only in the sensse that
        /// the object will always error, and not in the sense of crashing
        /// the process.
        #[allow(dead_code)]
        pub(crate) fn from_handle_internal(h: Handle) -> Self {
            Self(h)
        }
    };
}

macro_rules! pausable_common {
    () => {
        pub fn pause(&self) -> Result<()> {
            check_error(unsafe { syz_pause(self.to_syz_handle()) })
        }

        pub fn play(&self) -> Result<()> {
            check_error(unsafe { syz_play(self.to_syz_handle()) })
        }
    };
}

macro_rules! source_common {
    () => {
        pub fn add_generator<T: IsGenerator>(&self, generator: &T) -> Result<()> {
            check_error(unsafe {
                syz_sourceAddGenerator(self.to_syz_handle(), generator.to_syz_handle())
            })
        }

        pub fn remove_generator<T: IsGenerator>(&self, generator: &T) -> Result<()> {
            check_error(unsafe {
                syz_sourceRemoveGenerator(self.to_syz_handle(), generator.to_syz_handle())
            })
        }
    };
}

macro_rules! effect_common {
    () => {
        pub fn reset(&self) -> Result<()> {
            check_error(unsafe { syz_effectReset(self.to_syz_handle()) })
        }
    };
}
