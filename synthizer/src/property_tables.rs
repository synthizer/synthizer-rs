macro_rules! bool_p {
    ($syz_const: expr, $getter: ident, $setter: ident) => {
        pub fn $getter(&self) -> Result<bool> {
            let mut out = Default::default();
            check_error(unsafe {
                syz_getI(
                    &mut out as *mut i32,
                    self.to_syz_handle(),
                    $syz_const as i32,
                )
            })?;
            Ok(out != 0)
        }

        pub fn $setter(&self, value: bool) -> Result<()> {
            check_error(unsafe {
                syz_setI(self.to_syz_handle(), $syz_const as i32, value as i32)
            })?;
            Ok(())
        }
    };
}

macro_rules! enum_p {
    ($e: ty, $syz_const: expr, $getter: ident, $setter: ident) => {
        pub fn $getter(&self) -> Result<$e> {
            let mut out = Default::default();
            check_error(unsafe {
                syz_getI(
                    &mut out as *mut i32,
                    self.to_syz_handle(),
                    $syz_const as i32,
                )
            })?;
            Ok(unsafe { std::mem::transmute(out) })
        }

        pub fn $setter(&self, value: $e) -> Result<()> {
            check_error(unsafe {
                syz_setI(self.to_syz_handle(), $syz_const as i32, value as i32)
            })?;
            Ok(())
        }
    };
}

macro_rules! double_p {
    ($syz_const: expr, $getter: ident, $setter: ident) => {
        pub fn $getter(&self) -> Result<f64> {
            let mut out = Default::default();
            check_error(unsafe {
                syz_getD(
                    &mut out as *mut f64,
                    self.to_syz_handle(),
                    $syz_const as i32,
                )
            })?;
            Ok(out)
        }

        pub fn $setter(&self, value: f64) -> Result<()> {
            check_error(unsafe { syz_setD(self.to_syz_handle(), $syz_const as i32, value) })?;
            Ok(())
        }
    };
}

macro_rules! object_p {
    ($syz_const: expr, $setter: ident) => {
        pub fn $setter<T: ToSyzHandle>(&self, obj: &T) -> Result<()> {
            check_error(unsafe {
                syz_setO(self.to_syz_handle(), $syz_const as i32, obj.to_syz_handle())
            })?;
            Ok(())
        }
    };
}

macro_rules! biquad_p {
    ($syz_const: expr, $getter:ident, $setter: ident) => {
        pub fn $getter(&self) -> Result<BiquadConfig> {
            let mut out = Default::default();
            check_error(unsafe {
                syz_getBiquad(
                    &mut out as *mut syz_BiquadConfig,
                    self.to_syz_handle(),
                    $syz_const as i32,
                )
            })?;
            Ok(BiquadConfig { cfg: out })
        }

        pub fn $setter(&self, cfg: &BiquadConfig) -> Result<()> {
            check_error(unsafe {
                syz_setBiquad(
                    self.to_syz_handle(),
                    $syz_const as i32,
                    &cfg.cfg as *const syz_BiquadConfig,
                )
            })
        }
    };
}

macro_rules! double3_p {
    ($syz_const: expr, $getter: ident, $setter: ident) => {
        pub fn $getter(&self) -> Result<(f64, f64, f64)> {
            let mut o1 = Default::default();
            let mut o2 = Default::default();
            let mut o3 = Default::default();
            check_error(unsafe {
                syz_getD3(
                    &mut o1 as *mut f64,
                    &mut o2 as *mut f64,
                    &mut o3 as *mut f64,
                    self.to_syz_handle(),
                    $syz_const as i32,
                )
            })?;
            Ok((o1, o2, o3))
        }

        pub fn $setter(&self, values: (f64, f64, f64)) -> Result<()> {
            check_error(unsafe {
                syz_setD3(
                    self.to_syz_handle(),
                    $syz_const as i32,
                    values.0,
                    values.1,
                    values.2,
                )
            })
        }
    };
}

macro_rules! double6_p {
    ($syz_const: expr, $getter: ident, $setter: ident) => {
        pub fn $getter(&self) -> Result<(f64, f64, f64, f64, f64, f64)> {
            let mut o1 = Default::default();
            let mut o2 = Default::default();
            let mut o3 = Default::default();
            let mut o4 = Default::default();
            let mut o5 = Default::default();
            let mut o6 = Default::default();
            check_error(unsafe {
                syz_getD6(
                    &mut o1 as *mut f64,
                    &mut o2 as *mut f64,
                    &mut o3 as *mut f64,
                    &mut o4 as *mut f64,
                    &mut o5 as *mut f64,
                    &mut o6 as *mut f64,
                    self.to_syz_handle(),
                    $syz_const as i32,
                )
            })?;
            Ok((o1, o2, o3, o4, o5, o6))
        }

        pub fn $setter(&self, values: (f64, f64, f64, f64, f64, f64)) -> Result<()> {
            check_error(unsafe {
                syz_setD6(
                    self.to_syz_handle(),
                    $syz_const as i32,
                    values.0,
                    values.1,
                    values.2,
                    values.3,
                    values.4,
                    values.5,
                )
            })
        }
    };
}

macro_rules! generator_properties {
    () => {
        double_p!(SYZ_P_PITCH_BEND, get_pitch_bend, set_pitch_bend);
        double_p!(SYZ_P_GAIN, get_gain, set_gain);
    };
}

macro_rules! source_properties {
    () => {
        double_p!(SYZ_P_GAIN, get_gain, set_gain);
        biquad_p!(SYZ_P_FILTER, get_filter, set_filter);
        biquad_p!(SYZ_P_FILTER_DIRECT, get_filter_direct, set_filter_direct);
        biquad_p!(SYZ_P_FILTER_EFFECTS, get_filter_effects, set_filter_effects);
    };
}

macro_rules! effect_properties {
    () => {
        double_p!(SYZ_P_GAIN, get_gain, set_gain);
        biquad_p!(SYZ_P_FILTER_INPUT, get_filter_input, set_filter_input);
    };
}
