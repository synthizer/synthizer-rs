use synthizer_sys::*;

use crate::handle::*;
use crate::*;

/// A Synthizer route, used to connect sources to effects.  See
/// [Context::config_route] or, if you want to avoid this type,
/// [Context::config_route_simple].
///
/// The `Default` impl on this type is a route with default settings.  For more
/// advanced configuration, start with [RouteBuilder].
#[derive(Clone)]
pub struct RouteConfig(pub(crate) syz_RouteConfig);

/// A builder for an effect route.  `new` and `Default` are the equivalent of `syz_initRouteConfig`, e.g. a builder representing the default route.
#[derive(Clone)]
pub struct RouteConfigBuilder(syz_RouteConfig);

impl RouteConfigBuilder {
    pub fn new() -> RouteConfigBuilder {
        let mut cfg = Default::default();
        unsafe { syz_initRouteConfig(&mut cfg as *mut syz_RouteConfig) };
        RouteConfigBuilder(cfg)
    }

    pub fn set_gain(mut self, gain: f64) -> RouteConfigBuilder {
        self.0.gain = gain;
        self
    }

    pub fn set_fade_time(mut self, fade_time: f64) -> RouteConfigBuilder {
        self.0.fade_time = fade_time;
        self
    }

    pub fn set_filter(mut self, filter: BiquadConfig) -> RouteConfigBuilder {
        self.0.filter = filter.cfg;
        self
    }

    pub fn build(self) -> RouteConfig {
        RouteConfig(self.0)
    }
}

impl Default for RouteConfigBuilder {
    fn default() -> RouteConfigBuilder {
        RouteConfigBuilder::new()
    }
}

impl Default for RouteConfig {
    fn default() -> RouteConfig {
        RouteConfigBuilder::new().build()
    }
}

// Hack to stop people implementing our internal traits.
mod route_traits {
    use super::*;

    pub trait RouteInput: ToHandle {}
    pub trait RouteOutput: ToHandle {}
}
pub(crate) use route_traits::*;

impl RouteOutput for Handle {}
impl RouteOutput for DirectSource {}
impl RouteOutput for PannedSource {}
impl RouteOutput for Source3D {}

impl RouteInput for Handle {}
impl RouteInput for GlobalEcho {}
impl RouteInput for GlobalFdnReverb {}
