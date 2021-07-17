//! Test that userdata can be set, retrieved, and dropped at library shutdown.
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use synthizer as syz;

struct UserdataStruct(Arc<AtomicBool>);

impl Drop for UserdataStruct {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

#[test]
fn test_userdata_drops() -> Result<()> {
    let did_drop = Arc::new(AtomicBool::new(false));

    {
        let _guard = syz::initialize()?;
        let context = syz::Context::new()?;
        context.set_userdata(Some(UserdataStruct(did_drop.clone())))?;
        let ud = context.get_userdata()?;
        let ud = ud.expect("Should have userdata");
        assert!(ud.downcast::<UserdataStruct>().is_ok());
    }

    assert!(did_drop.load(Ordering::Relaxed));

    Ok(())
}
