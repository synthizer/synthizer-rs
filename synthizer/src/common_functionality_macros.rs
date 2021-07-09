//! Define the "base classes" through macro magic.E.g. `source_common` is the
//! source "base class".
use synthizer_sys::*;

use crate::errors::*;
use crate::generator::Generator;
use crate::handle::*;
use crate::*;

macro_rules! pausable_common {
    ($t: ty) => {
        impl $t {
            pub fn pause(&self) -> Result<()> {
                check_error(unsafe { syz_pause(self.to_handle()) })
            }

            pub fn play(&self) -> Result<()> {
                check_error(unsafe { syz_play(self.to_handle()) })
            }
        }
    };
}

macro_rules! source_common {
    ($t: ty) => {
        impl $t {
            pub fn add_generator<T: Generator>(&self, generator: &T) -> Result<()> {
                check_error(unsafe {
                    syz_sourceAddGenerator(self.to_handle(), generator.to_handle())
                })
            }

            pub fn remove_generator<T: Generator>(&self, generator: &T) -> Result<()> {
                check_error(unsafe {
                    syz_sourceRemoveGenerator(self.to_handle(), generator.to_handle())
                })
            }
        }
    };
}

macro_rules! effect_common {
    ($t: ty) => {
        impl $t {
            pub fn reset(&self) -> Result<()> {
                check_error(unsafe { syz_effectReset(self.to_handle()) })
            }
        }
    };
}

pausable_common!(BufferGenerator);
pausable_common!(Context);
pausable_common!(DirectSource);
pausable_common!(NoiseGenerator);
pausable_common!(PannedSource);
pausable_common!(Source3D);
pausable_common!(StreamingGenerator);

source_common!(DirectSource);
source_common!(PannedSource);
source_common!(Source3D);

effect_common!(GlobalEcho);

effect_common!(GlobalFdnReverb);
