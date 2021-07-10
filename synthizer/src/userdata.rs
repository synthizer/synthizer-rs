//! A userdata implementation.
//!
//! Synthizer offers a `syz_handleGetUserdata` and similar, but these are
//! insufficient for Rust, which additionally wishes to offer type-level thread
//! safety guarantees.    Specifically, userdata is destroyed after Synthizer is
//! deinitialized, and the only way to guarantee that can't ever happen is to
//! make larger programs deal with having a Synthizer lifetime everywhere which
//! might use a handle (so, e.g., it would become impossible to pass handles
//! between threads).  To do this, we fake out the functionality.  Using
//! `RwLock` and `ArcSwap`, it is possible to almost guarantee wait-free
//! userdata maps.  The map in this module is keyed by the integer value of the
//! handle, and holds userdata.
use synthizer_sys::*;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// How many empty userdatas are we willing to leave in the map before garbage
/// collecting it?  This constant controls amortization of blocking.
/// Essentially, we set it high enough such that if an app has this many
/// concurrent Synthizer objects, the app is having much larger issues than the
/// `Handle`'s `Drop` impl blocking.
const USERDATA_GC_THRESHOLD: usize = 16384;

struct UserdataCell {
    value: Option<Arc<dyn std::any::Any + Send + Sync>>,
    refcount: usize,
}

#[derive(Default)]
struct UserdataMap {
    inner: HashMap<syz_Handle, RwLock<UserdataCell>>,
}

static mut USERDATA_MAP: *mut RwLock<UserdataMap> = std::ptr::null_mut();

/// We can statically guarantee that no user can get a handle before this
/// function is called because getting a handle requires Synthizer to have
/// successfully initialized.
pub(crate) fn init_userdata_map() {
    unsafe {
        USERDATA_MAP = Box::into_raw(Box::new(RwLock::new(UserdataMap::new())));
    }
}

/// We can clear the userdata map at shutdown, as long as we don't actually
/// delete the map itself: anyone who reads it after jhust stops seeing
/// userdata.
pub(crate) fn clear_userdata_map() {
    unsafe {
        (&*USERDATA_MAP)
            .write()
            .expect("Mutex poisoned")
            .inner
            .clear();
    }
}

impl UserdataMap {
    fn new() -> UserdataMap {
        Default::default()
    }

    fn gc(&mut self) {
        self.inner.retain(|_, v| {
            let rguard = v.read().expect("Mutex poisoned");
            rguard.refcount != 0
        });
    }

    // Register a handle with the map or increment the reference count if it was
    // already registered.
    fn register_handle(&mut self, handle: syz_Handle) {
        self.inner
            .entry(handle)
            .and_modify(|v| v.write().expect("Mutex poisoned").refcount += 1)
            .or_insert_with(|| {
                RwLock::new(UserdataCell {
                    value: None,
                    refcount: 1,
                })
            });
    }

    /// Returns whether the map wants to run gc.
    fn unregister_handle(&self, h: syz_Handle) -> bool {
        let mut guard = self
            .inner
            .get(&h)
            .expect("Handles are always inserted")
            .write()
            .expect("Mutex poisoned");
        guard.refcount -= 1;
        if guard.refcount == 0 {
            guard.value = None;
        }
        self.inner.len() > USERDATA_GC_THRESHOLD
    }

    fn get_userdata(&self, h: syz_Handle) -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        self.inner
            .get(&h)
            .expect("Handles are always initialized")
            .read()
            .expect("Mutex poisoned")
            .value
            .clone()
    }

    fn set_userdata(&self, h: syz_Handle, userdata: Option<impl 'static + Send + Sync>) {
        self.inner
            .get(&h)
            .expect("Handles should always be registered")
            .write()
            .expect("Mutex poisoned")
            .value = userdata.map(|u| Arc::new(u) as Arc<dyn std::any::Any + Send + Sync>)
    }
}

pub(crate) fn register_handle(h: syz_Handle) {
    unsafe {
        (&*USERDATA_MAP)
            .write()
            .expect("Mutex poisoned")
            .register_handle(h)
    }
}

pub(crate) fn unregister_handle(h: syz_Handle) {
    let will_gc = unsafe {
        (&*USERDATA_MAP)
            .read()
            .expect("Mutex poisoned")
            .unregister_handle(h)
    };
    if will_gc {
        unsafe { (&*USERDATA_MAP).write().expect("Mutex poisoned").gc() }
    }
}

pub(crate) fn get_userdata(h: syz_Handle) -> Option<Arc<dyn std::any::Any + Send + Sync>> {
    unsafe {
        (&*USERDATA_MAP)
            .read()
            .expect("Mutex poisoned")
            .get_userdata(h)
    }
}

pub(crate) fn set_userdata(h: syz_Handle, value: Option<impl std::any::Any + Send + Sync>) {
    unsafe {
        (&*USERDATA_MAP)
            .read()
            .expect("Mutex Poisoned")
            .set_userdata(h, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reimplementation of `get_userdata` on objects for testing purposes.
    fn read_map<T: std::any::Any + Send + Sync>(
        map: &mut UserdataMap,
        h: syz_Handle,
    ) -> Option<Arc<T>> {
        map.get_userdata(h)?.downcast::<T>().ok()
    }

    #[test]
    fn test_userdata() {
        let mut map = UserdataMap::new();

        for _ in 1..10 {
            for i in 1..=10 {
                map.register_handle(i);
                map.set_userdata(i, Some(i));
                assert_eq!(*read_map::<syz_Handle>(&mut map, i).unwrap(), i);
            }
        }

        // Garbage collecting the map shouldn't change anything.
        map.gc();
        assert_eq!(map.inner.len(), 10);

        for attempt in (1..10usize).rev() {
            for i in 1..=10 {
                map.unregister_handle(i);
                if attempt != 1 {
                    assert_eq!(*read_map::<syz_Handle>(&mut map, i).unwrap(), i);
                } else {
                    assert_eq!(read_map::<syz_Handle>(&mut map, i), None);
                }
            }
        }

        map.gc();
        assert_eq!(map.inner.len(), 0);
    }
}
