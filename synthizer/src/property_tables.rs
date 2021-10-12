macro_rules! bool_p {
    ($syz_const: expr, $name: ident) => {
        pub fn $name(&self) -> BoolProperty {
            BoolProperty::new(self, $syz_const as i32)
        }
    };
}

macro_rules! enum_p {
    ($e: ty, $syz_const: expr, $name: ident) => {
        pub fn $name(&self) -> EnumProperty<$e> {
            EnumProperty::new(self, $syz_const as i32)
        }
    };
}

macro_rules! double_p {
    ($syz_const: expr, $name: ident) => {
        pub fn $name(&self) -> DoubleProperty {
            DoubleProperty::new(self, $syz_const as i32)
        }
    };
}

macro_rules! object_p {
    ($syz_const: expr, $name: ident) => {
        pub fn $name(&self) -> ObjectProperty {
            ObjectProperty::new(self, $syz_const as i32)
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
    ($syz_const: expr, $name: ident) => {
        pub fn $name(&self) -> Double3Property {
            Double3Property::new(self, $syz_const as i32)
        }
    };
}

macro_rules! double6_p {
    ($syz_const: expr, $name: ident) => {
        pub fn $name(&self) -> Double6Property {
            Double6Property::new(self, $syz_const as i32)
        }
    };
}

macro_rules! generator_properties {
    () => {
        double_p!(SYZ_P_PITCH_BEND, pitch_bend);
        double_p!(SYZ_P_GAIN, gain);
    };
}

macro_rules! source_properties {
    () => {
        double_p!(SYZ_P_GAIN, gain);
        biquad_p!(SYZ_P_FILTER, get_filter, set_filter);
        biquad_p!(SYZ_P_FILTER_DIRECT, get_filter_direct, set_filter_direct);
        biquad_p!(SYZ_P_FILTER_EFFECTS, get_filter_effects, set_filter_effects);
    };
}

macro_rules! effect_properties {
    () => {
        double_p!(SYZ_P_GAIN, gain);
        biquad_p!(SYZ_P_FILTER_INPUT, get_filter_input, set_filter_input);
    };
}
