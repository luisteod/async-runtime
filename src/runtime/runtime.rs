use crate::runtime::{executor::Executor, spawner::Spawner};

pub struct Runtime {
    pub executor: Executor,
    pub spawner: Spawner,
}

impl Runtime {
    pub fn start(&self) {
        self.executor.start();
    }
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        self.spawner.spawn(future);
    }
}
