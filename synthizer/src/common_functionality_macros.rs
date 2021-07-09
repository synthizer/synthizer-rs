//! Define the "base classes" through macro magic.E.g. `source_common` is the
//! source "base class".

macro_rules! pausable_common {
    () => {
        pub fn pause(&self) -> Result<()> {
            check_error(unsafe { syz_pause(self.to_handle()) })
        }

        pub fn play(&self) -> Result<()> {
            check_error(unsafe { syz_play(self.to_handle()) })
        }
    };
}

macro_rules! source_common {
    () => {
        pub fn add_generator<T: Generator>(&self, generator: &T) -> Result<()> {
            check_error(unsafe { syz_sourceAddGenerator(self.to_handle(), generator.to_handle()) })
        }

        pub fn remove_generator<T: Generator>(&self, generator: &T) -> Result<()> {
            check_error(unsafe {
                syz_sourceRemoveGenerator(self.to_handle(), generator.to_handle())
            })
        }
    };
}

macro_rules! effect_common {
    () => {
        pub fn reset(&self) -> Result<()> {
            check_error(unsafe { syz_effectReset(self.to_handle()) })
        }
    };
}
