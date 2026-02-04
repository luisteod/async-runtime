use mio::event::Source;

pub struct Registration {}

impl Registration {
    pub fn register(source: &impl Source) -> Self {
        // Get the IoDriver via global context and register
        Registration {}
    }
}
