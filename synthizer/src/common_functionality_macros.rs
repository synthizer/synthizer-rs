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
        pub fn add_generator<T: Generator>(&self, generator: &T) -> Result<()> {
            check_error(unsafe {
                syz_sourceAddGenerator(self.to_syz_handle(), generator.to_syz_handle())
            })
        }

        pub fn remove_generator<T: Generator>(&self, generator: &T) -> Result<()> {
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
