use crate::runtime::{executor::ExecutorHandle, io::scheduled_io::ScheduledIo};
use mio::event::Source;
use std::rc::Rc;

pub struct Registration {
    io: Rc<ScheduledIo>,
}

impl Registration {
    pub fn register(source: &mut impl Source, exec_handle: Rc<ExecutorHandle>) -> Self {
        // Get the IoDriver via global context and register
        let io = exec_handle.io().add_source(source);
        Registration { io }
    }
}
