//! The `Context`.
use crate::internal_prelude::*;

/// The `Context` represents an audio device.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct Context(pub(crate) Handle);

impl Context {
    pub fn new() -> Result<Context> {
        wrap_constructor(|ud, cb| {
            let mut h = 0;
            check_error(unsafe { syz_createContext(&mut h as *mut syz_Handle, ud, Some(cb)) })?;
            Ok(Context(Handle::new(h)))
        })
    }

    pub fn enable_events(&self) -> Result<()> {
        check_error(unsafe { syz_contextEnableEvents(self.to_syz_handle()) })
    }

    /// Get any pending events.  The returned iterator will not block, and
    /// iterates over any pending events until the first error.  This is lazy:
    /// to limit the number of events received, use `.take`.
    #[allow(clippy::needless_lifetimes)] // Actually appears to be a false positive.
    pub fn get_events<'a>(&'a self) -> impl Iterator<Item = Result<events::Event>> + 'a {
        events::EventIterator {
            context: self,
            errored: false,
        }
    }

    // Configure a route given a [RouteConfig], which can be constructed with a
    // [RouteConfigBuilder].  Corresponds to the `syz_initRouteConfig` and
    // `syz_routingConfigRoute` flow.
    pub fn config_route(
        &self,
        output: &dyn RouteOutput,
        input: &dyn RouteInput,
        config: &RouteConfig,
    ) -> Result<()> {
        check_error(unsafe {
            syz_routingConfigRoute(
                self.to_syz_handle(),
                output.to_syz_handle(),
                input.to_syz_handle(),
                &config.0 as *const syz_RouteConfig,
            )
        })?;
        Ok(())
    }

    /// Configure a route with the default settings.
    pub fn config_route_simple(
        &self,
        output: &dyn RouteOutput,
        input: &dyn RouteInput,
    ) -> Result<()> {
        self.config_route(output, input, &Default::default())
    }

    pub fn remove_route(
        &self,
        output: &dyn RouteOutput,
        input: &dyn RouteInput,
        fade_out: f64,
    ) -> Result<()> {
        check_error(unsafe {
            syz_routingRemoveRoute(
                self.to_syz_handle(),
                output.to_syz_handle(),
                input.to_syz_handle(),
                fade_out,
            )
        })
    }

    double_p!(SYZ_P_GAIN, gain);
    enum_p!(
        PannerStrategy,
        SYZ_P_DEFAULT_PANNER_STRATEGY,
        default_panner_strategy
    );
    enum_p!(
        DistanceModel,
        SYZ_P_DEFAULT_DISTANCE_MODEL,
        default_distance_model
    );
    double_p!(SYZ_P_DEFAULT_DISTANCE_REF, default_distance_ref);
    double_p!(SYZ_P_DEFAULT_DISTANCE_MAX, default_distance_max);
    double_p!(SYZ_P_DEFAULT_ROLLOFF, default_rolloff);
    double_p!(SYZ_P_DEFAULT_CLOSENESS_BOOST, default_closeness_boost);
    double_p!(
        SYZ_P_DEFAULT_CLOSENESS_BOOST_DISTANCE,
        default_closeness_boost_distance
    );
    double3_p!(SYZ_P_POSITION, position);
    double6_p!(SYZ_P_ORIENTATION, orientation);

    object_common!();
    pausable_common!();
}

handle_traits!(Context);
