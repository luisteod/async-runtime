use super::scheduled_io::ScheduledIo;
use mio::{Events, Interest, Poll, Registry, Token, event::Source};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Driver that drives IO events. 
/// It's backed by OS selector primitives, such as epoll(7) provided by [`mio`].
/// The driver holds an event loop which watches for IO events and dispatches 
/// calling `wake` for each interested contained in the HashMap.

const MAX_EVENTS_CAPACITY: usize = 1024;
const DEFAULT_IO_INTERESTS: Interest = Interest::READABLE.add(Interest::WRITABLE);

pub struct IoDriver {
    poll: Poll,
}

pub struct IoHandle {
    registry: Registry,
    scheduled_io_map: RefCell<HashMap<Token, Rc<ScheduledIo>>>,
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

    pub fn park(&mut self, handle: &IoHandle) {
        let mut events = Events::with_capacity(MAX_EVENTS_CAPACITY);
        self.poll
            .poll(&mut events, None)
            .expect("Error polling IO events");

        for event in &events {
            let token = event.token();

            let map_guard = handle.scheduled_io_map.borrow();
            let io = map_guard.get(&token);

            io.expect("No IO founded").wake();
        }
    }
}

impl IoHandle {
    pub fn add_source<E>(&self, source: &mut E) -> Rc<ScheduledIo>
    where
        E: Source,
    {
        let scheduled_io = Rc::new(ScheduledIo::default());
        let token = scheduled_io.token();

        self.scheduled_io_map
            .borrow_mut()
            .insert(token, scheduled_io.clone());

        let _ = self.registry.register(source, token, DEFAULT_IO_INTERESTS);

        scheduled_io
    }
}
