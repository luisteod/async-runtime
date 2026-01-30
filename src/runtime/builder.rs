use std::sync::mpsc::sync_channel;

use crate::runtime::{executor::Executor, runtime::Runtime, spawner::Spawner};

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor::new(ready_queue), Spawner { task_sender })
}

pub fn build() -> Runtime {
    let (executor, spawner) = new_executor_and_spawner();
    Runtime { executor, spawner }
}
