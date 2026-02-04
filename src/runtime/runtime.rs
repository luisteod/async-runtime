use std::rc::Rc;

use crate::runtime::{
    context,
    executor::{ExecutorDriver, ExecutorHandle},
    spawner::Spawner,
};

pub struct Runtime {
    executor_driver: ExecutorDriver,
    executor_handle: Rc<ExecutorHandle>,
    spawner: Spawner,
}

impl Runtime {
    pub fn new(exe_driver: ExecutorDriver, exe_handle: ExecutorHandle, spawner: Spawner) -> Self {
        Runtime {
            executor_driver: exe_driver,
            executor_handle: Rc::new(exe_handle),
            spawner,
        }
    }

    pub fn start(&mut self) {
        // Sends the driver handle to the global context
        context::enter_runtime(&self.executor_handle);
        self.executor_driver.start();
    }
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        self.spawner.spawn(future);
    }
}
