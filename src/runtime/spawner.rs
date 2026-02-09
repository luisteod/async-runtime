use std::{
    future::Future,
    sync::mpsc::SyncSender,
    sync::{Arc, Mutex},
};

use crate::runtime::task::Task;
use futures::future::FutureExt;

/// `Spawner` spawns new futures onto the task channel.
pub struct Spawner {
    pub task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender
            .try_send(task)
            .expect("too many tasks queued");
    }
}
