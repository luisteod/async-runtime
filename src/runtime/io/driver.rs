use super::scheduled_io::ScheduledIo;
use mio::{Events, Interest, Poll, Registry, Token, event::Source};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Driver that drives IO events.
/// It's backed by OS selector primitives, such as epoll(7) provided by [`mio`].
/// The driver holds an event loop which watches for IO events and dispatches
/// calling `wake` for each interested contained in the HashMap.

const MAX_EVENTS_CAPACITY: usize = 1024;

pub struct IoDriver {
    poll: Poll,
}

pub struct IoHandle {
    registry: Registry,
    scheduled_io_map: RefCell<HashMap<Token, Arc<ScheduledIo>>>,
}

impl IoDriver {
    pub fn new() -> (IoDriver, IoHandle) {
        let poll = Poll::new().expect("Error creating Poll instance");

        let registry = poll.registry().try_clone().expect("Error cloning registry");
        let scheduled_io_map = RefCell::new(HashMap::new());

        let handle = IoHandle {
            registry,
            scheduled_io_map,
        };
        let driver = IoDriver { poll };

        (driver, handle)
    }

    pub fn park(&mut self, handle: &IoHandle, timeout: Duration) {
        let mut events = Events::with_capacity(MAX_EVENTS_CAPACITY);
        self.poll
            .poll(&mut events, Some(timeout))
            .expect("Error polling IO events");

        for event in &events {
            let token = event.token();

            let io_ref = {
                let map = handle.scheduled_io_map.borrow();
                map.get(&token).cloned()
            };

            if let Some(io) = io_ref {
                io.set_event(event.clone());
                io.wake();
            }
        }
    }
}

impl IoHandle {
    pub fn add_source<E>(&self, source: &mut E, interest: Interest) -> Arc<ScheduledIo>
    where
        E: Source,
    {
        let scheduled_io = Arc::new(ScheduledIo::default());
        let token = scheduled_io.token();

        self.scheduled_io_map
            .borrow_mut()
            .insert(token, scheduled_io.clone());

        let _ = self.registry.register(source, token, interest);

        scheduled_io
    }
}
