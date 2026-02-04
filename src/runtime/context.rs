use crate::runtime::executor::ExecutorHandle;
use std::cell::OnceCell;
use std::rc::Rc;

/// A global variable that holds a Rc reference of the [`ExecutorHandle`] for
/// easy access in a global manner.
/// When this executor start's an execution, it must calls [`enter_runtime`]
/// to populate this [`CURRENT_CONTEXT`].

struct Context {
    executor: OnceCell<Rc<ExecutorHandle>>,
}

thread_local! {
    static CURRENT_CONTEXT: Context = const {
        Context {
            executor: OnceCell::new()
        }
    };
}

impl Context {
    fn set_executor(&self, executor: &Rc<ExecutorHandle>) {
        if self.executor.set(executor.clone()).is_err() {
            panic!("Runtime error: enter_runtime called twice on the same thread");
        }
    }

    fn get_executor_handle(&self) -> Option<Rc<ExecutorHandle>> {
        let executor = self.executor.get().cloned();

        executor
    }
}

pub fn enter_runtime(executor: &Rc<ExecutorHandle>) {
    CURRENT_CONTEXT.with(|ctx| {
        ctx.set_executor(executor);
    })
}

pub fn get_executor_handle() -> Rc<ExecutorHandle> {
    CURRENT_CONTEXT.with(|ctx| {
        ctx.get_executor_handle()
            .expect("Context error: get_executor called outside of a runtime context")
    })
}
