use synthizer_sys::*;

use crate::errors::*;
use crate::handle::*;
use crate::*;

macro_rules! bool_p {
    ($t: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<bool> {
                let mut out = Default::default();
                check_error(unsafe {
                    syz_getI(&mut out as *mut i32, self.to_handle(), $syz_const as i32)
                })?;
                Ok(out != 0)
            }

            pub fn $setter(&self, value: bool) -> Result<()> {
                check_error(unsafe {
                    syz_setI(self.to_handle(), $syz_const as i32, value as i32)
                })?;
                Ok(())
            }
        }
    };
}

macro_rules! double_p {
    ($t: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        impl $t {
            pub fn $getter(&self) -> Result<f64> {
                let mut out = Default::default();
                check_error(unsafe {
                    syz_getD(&mut out as *mut f64, self.to_handle(), $syz_const as i32)
                })?;
                Ok(out)
            }

            pub fn $setter(&self, value: f64) -> Result<()> {
                check_error(unsafe { syz_setD(self.to_handle(), $syz_const as i32, value) })?;
                Ok(())
            }
        }
    };
}

macro_rules! object_p {
    ($t: ty, $syz_const: expr, $setter: ident) => {
        impl $t {
            pub fn $setter<T: ToHandle>(&self, obj: &T) -> Result<()> {
                check_error(unsafe {
                    syz_setO(self.to_handle(), $syz_const as i32, obj.to_handle())
                })?;
                Ok(())
            }
        }
    };
}

macro_rules! generator_common {
    ($t: ty) => {
        double_p!($t, SYZ_P_PITCH_BEND, get_pitch_bend, set_pitch_bend);
        double_p!($t, SYZ_P_GAIN, get_gain, set_gain);
    };
}

generator_common!(BufferGenerator);
bool_p!(BufferGenerator, SYZ_P_LOOPING, get_looping, set_looping);
double_p!(
    BufferGenerator,
    SYZ_P_PLAYBACK_POSITION,
    get_playback_position,
    set_playback_position
);
object_p!(BufferGenerator, SYZ_P_BUFFER, set_buffer);
