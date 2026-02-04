use futures::{future::BoxFuture, task::ArcWake};
use std::{
    sync::mpsc::SyncSender,
    sync::{Arc, Mutex},
};

/// A future that can reschedule itself to be polled by an `Executor`.
pub struct Task {
    pub future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    pub task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .try_send(cloned)
            .expect("too many tasks queued");
    }
}
