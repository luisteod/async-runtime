use futures::task::waker_ref;
use std::{
    sync::{Arc, mpsc::Receiver},
    task::{Context, Poll},
    time::Duration,
};

use crate::runtime::{
    io::{IoDriver, IoHandle},
    task::Task,
};

const POLL_TIMEOUT: Duration = Duration::from_millis(100);

/// Task executor that receives tasks off of a channel and runs them.
pub struct ExecutorDriver {
    io_driver: IoDriver,
    ready_queue: Receiver<Arc<Task>>,
    metadata: ExecutionData,
}

struct ExecutionData {
    pending_tasks: i32,
}

pub struct ExecutorHandle {
    io_handle: IoHandle,
}

impl ExecutorHandle {
    pub fn io(&self) -> &IoHandle {
        &self.io_handle
    }
}

impl ExecutorDriver {
    pub fn new(ready_queue: Receiver<Arc<Task>>) -> (ExecutorDriver, ExecutorHandle) {
        let (io_driver, io_handle) = IoDriver::new();

        let metadata = ExecutionData { pending_tasks: 0 };

        let exec_driver = ExecutorDriver {
            io_driver,
            ready_queue,
            metadata,
        };

        let exec_handle = ExecutorHandle {
            io_handle: io_handle,
        };

        (exec_driver, exec_handle)
    }

    pub fn start(mut self, handle: &ExecutorHandle) {
        loop {
            // Try receive tasks from the channel
            self.process_tasks();

            let poll_timeout = if self.metadata.pending_tasks > 0 {
                Duration::ZERO
            } else {
                POLL_TIMEOUT
            };

            // Park in IO driver waiting for IO events for a timeout
            self.io_driver.park(&handle.io_handle, poll_timeout);
        }
    }

    fn process_tasks(&mut self) {
        while let Ok(task) = self.ready_queue.try_recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                match future.as_mut().poll(context) {
                    Poll::Pending => {
                        *future_slot = Some(future);
                        self.metadata.pending_tasks += 1;
                    }
                    Poll::Ready(_) => {
                        self.metadata.pending_tasks -= 1;
                    }
                }

                if self.metadata.pending_tasks == 0 {
                    break;
                }
            }
        }
    }
}
