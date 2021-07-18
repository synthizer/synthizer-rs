use crate::internal_prelude::*;

#[non_exhaustive]
#[repr(i32)]
pub enum EventType {
    // Exposed only because Rust doesn't allow having private enum variants.
    Invalid = SYZ_EVENT_TYPE_INVALID as i32,
    Finished = SYZ_EVENT_TYPE_FINISHED as i32,
    Looped = SYZ_EVENT_TYPE_LOOPED as i32,
}

pub struct Event {
    pub source: Handle,
    pub context: Option<Context>,
    pub r#type: EventType,
}

/// An iterator over events. Internal only.  We expose this through impl trait.
pub(crate) struct EventIterator<'a> {
    pub(crate) context: &'a Context,
    pub(crate) errored: bool,
}

impl<'a> EventIterator<'a> {
    fn get_next(&self) -> Result<Option<Event>> {
        // These record whether or not we got the handle of the appropriate
        // type, and make sure to drop the value if we got as far as
        // incrementing the reference.
        let mut source: Option<Handle> = None;
        let mut context: Option<Context> = None;

        let mut evt: syz_Event = Default::default();

        check_error(unsafe {
            syz_contextGetNextEvent(&mut evt as *mut syz_Event, self.context.to_syz_handle(), 0)
        })?;

        if evt.type_ == SYZ_EVENT_TYPE_INVALID as i32 {
            return Ok(None);
        }

        // be careful here: we must make sure to deinitialize the event before
        // returning.
        let inc_ret = check_error(unsafe { syz_handleIncRef(evt.source) })
            .and_then(|_| {
                source = Some(Handle::new(evt.source));
                if evt.context == 0 {
                    return Ok(());
                }
                check_error(unsafe { syz_handleIncRef(evt.context) })
            })
            .map(|_| {
                context = Some(Context(Handle::new(evt.context)));
            });

        let r#type = unsafe { std::mem::transmute(evt.type_) };

        unsafe { syz_eventDeinit(&mut evt as *mut syz_Event) };
        inc_ret?;

        Ok(Some(Event {
            // We always have a source.
            source: source.unwrap(),
            context,
            r#type,
        }))
    }
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Result<Event>> {
        if self.errored {
            return None;
        }

        match self.get_next() {
            Ok(Some(e)) => Some(Ok(e)),
            Ok(None) => None,
            Err(e) => {
                self.errored = true;
                Some(Err(e))
            }
        }
    }
}
