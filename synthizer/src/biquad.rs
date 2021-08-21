use crate::internal_prelude::*;

/// Suggested default Q for filter design functions.  If you don't have a better
/// idea what value of Q to use, this is what you want.
pub const DEFAULT_Q: f64 = 0.7071135624381276;

/// Biquad filters.  This is used primarily as the value to biquad properties.
/// The member functions match the Synthizer C functions of the same name.
///
/// Since Rust doesn't have default parameters, you can use `DEFAULT_Q` for
/// Synthizer's suggested default Q.
///
/// The `Default` impl on this struct returns an identity filter designed with
/// `syz_biquadDesignIdentity`, a filter which passes audio through unchanged.
#[derive(Copy, Clone, Debug)]
pub struct BiquadConfig {
    pub(crate) cfg: syz_BiquadConfig,
}

impl BiquadConfig {
    pub fn design_identity() -> Result<BiquadConfig> {
        let mut cfg: syz_BiquadConfig = Default::default();
        check_error(unsafe { syz_biquadDesignIdentity(&mut cfg as *mut syz_BiquadConfig) })?;
        Ok(BiquadConfig { cfg })
    }

    pub fn design_lowpass(frequency: f64, q: f64) -> Result<BiquadConfig> {
        let mut cfg = Default::default();
        check_error(unsafe {
            syz_biquadDesignLowpass(&mut cfg as *mut syz_BiquadConfig, frequency, q)
        })?;
        Ok(BiquadConfig { cfg })
    }

    pub fn design_highpass(frequency: f64, q: f64) -> Result<BiquadConfig> {
        let mut cfg: syz_BiquadConfig = Default::default();
        check_error(unsafe {
            syz_biquadDesignHighpass(&mut cfg as *mut syz_BiquadConfig, frequency, q)
        })?;
        Ok(BiquadConfig { cfg })
    }

    pub fn design_bandpass(frequency: f64, bw: f64) -> Result<BiquadConfig> {
        let mut cfg: syz_BiquadConfig = Default::default();
        check_error(unsafe {
            syz_biquadDesignBandpass(&mut cfg as *mut syz_BiquadConfig, frequency, bw)
        })?;
        Ok(BiquadConfig { cfg })
    }
}
