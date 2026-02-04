use futures::task::waker_ref;
use std::{
    sync::{Arc, mpsc::Receiver},
    task::{Context, Poll},
};

use crate::runtime::{
    io::{IoDriver, IoHandle},
    task::Task,
};

/// Task executor that receives tasks off of a channel and runs them.
pub struct ExecutorDriver {
    pending_tasks: i32,
    io_driver: IoDriver,
    ready_queue: Receiver<Arc<Task>>,
}

pub struct ExecutorHandle {
    io_handle: IoHandle,
}

impl ExecutorDriver {
    pub fn new(ready_queue: Receiver<Arc<Task>>) -> (ExecutorDriver, ExecutorHandle) {
        let (io_driver, io_handle) = IoDriver::new();

        let driver = ExecutorDriver {
            pending_tasks: 0,
            io_driver: io_driver,
            ready_queue,
        };

        let handle = ExecutorHandle {
            io_handle: io_handle,
        };

        (driver, handle)
    }

    pub fn start(&mut self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                match future.as_mut().poll(context) {
                    Poll::Pending => {
                        *future_slot = Some(future);
                        self.pending_tasks += 1;
                    }
                    Poll::Ready(_) => {
                        self.pending_tasks -= 1;
                    }
                }

                if self.pending_tasks == 0 {
                    break;
                }
            }
        }
    }
}
