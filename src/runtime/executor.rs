use std::{
    sync::{Arc, mpsc::Receiver},
    task::Context
};
use futures::task::waker_ref;

use crate::runtime::task::Task;

/// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    pub ready_queue: Receiver<Arc<Task>>
}

impl Executor {
    pub fn start(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
            
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}