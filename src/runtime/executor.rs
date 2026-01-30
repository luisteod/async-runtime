use futures::task::waker_ref;
use std::{
    cell::Cell,
    sync::{Arc, mpsc::Receiver},
    task::Context,
    task::Poll
};

use crate::runtime::task::Task;

/// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    pending_tasks: Cell<i32>,

    pub ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn new(ready_queue: Receiver<Arc<Task>>) -> Self {
        Executor {
            pending_tasks: Cell::new(0),
            ready_queue,
        }
    }

    pub fn start(&self) {
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
                        self.pending_tasks.set(self.pending_tasks.get() + 1);
                    }
                    Poll::Ready(_) => {
                        self.pending_tasks.set(self.pending_tasks.get() - 1);
                    }
                }

                if self.pending_tasks.get() == 0 { 
                    break
                }
            }
        }
    }
}
