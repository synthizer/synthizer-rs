use crate::internal_prelude::*;

#[derive(Debug, Clone)]
pub struct DeleteBehaviorConfig {
    pub(crate) cfg: syz_DeleteBehaviorConfig,
}

impl Default for DeleteBehaviorConfig {
    fn default() -> DeleteBehaviorConfig {
        let mut cfg = std::mem::MaybeUninit::<syz_DeleteBehaviorConfig>::uninit();
        unsafe { syz_initDeleteBehaviorConfig(cfg.as_mut_ptr()) };
        DeleteBehaviorConfig {
            cfg: unsafe { cfg.assume_init() },
        }
    }
}

impl DeleteBehaviorConfig {
    pub fn new() -> DeleteBehaviorConfig {
        Default::default()
    }

    pub fn set_linger(&mut self, linger: bool) {
        self.cfg.linger = linger as std::os::raw::c_int;
    }

    pub fn set_linger_timeout(&mut self, timeout: f64) {
        self.cfg.linger_timeout = timeout;
    }
}

///A builder for a `DeleteBehaviorConfig`.
#[derive(Debug, Default)]
pub struct DeleteBehaviorConfigBuilder {
    cfg: DeleteBehaviorConfig,
}

impl DeleteBehaviorConfigBuilder {
    pub fn new() -> DeleteBehaviorConfigBuilder {
        Default::default()
    }

    pub fn linger(mut self, linger: bool) -> Self {
        self.cfg.set_linger(linger);
        self
    }

    pub fn linger_timeout(mut self, timeout: f64) -> Self {
        self.cfg.set_linger_timeout(timeout);
        self
    }

    pub fn build(self) -> DeleteBehaviorConfig {
        self.cfg
    }
}
