use crate::runtime::{executor::ExecutorHandle, io::scheduled_io::ScheduledIo};
use mio::Interest;
use mio::event::Source;
use std::io;
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

    pub(crate) async fn async_io<R>(
        &self,
        interest: Interest,
        mut f: impl FnMut() -> io::Result<R>,
    ) -> io::Result<R> {
        loop {
            // This now returns an owned Future that we can await
            let _event = ScheduledIo::io_future(&self.io, interest).await;

            match f() {
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // The OS said "try again later", so we loop and await again
                    continue;
                }
                x => return x,
            }
        }
    }
}
