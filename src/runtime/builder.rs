use std::sync::mpsc::sync_channel;

use crate::runtime::{
    executor::{ExecutorDriver, ExecutorHandle},
    runtime::Runtime,
    spawner::Spawner,
};

// Maximum number of tasks to allow queueing in the channel at once.
// This is just to make `sync_channel` happy, and wouldn't be present in
// a real executor.
const MAX_QUEUED_TASKS: usize = 10_000;

fn new_executor_and_spawner() -> (ExecutorDriver, ExecutorHandle, Spawner) {
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    let (exe_driver, exe_handle) = ExecutorDriver::new(ready_queue);
    let spawner = Spawner { task_sender };

    (exe_driver, exe_handle, spawner)
}

pub fn build() -> Runtime {
    let (exe_driver, exe_handle, spawner) = new_executor_and_spawner();
    Runtime::new(exe_driver, exe_handle, spawner)
}
