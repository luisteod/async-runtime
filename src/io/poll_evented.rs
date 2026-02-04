use crate::runtime::{context, io::Registration};
use mio::event::Source;

pub struct PollEvented<E: Source> {
    io: E,
    registration: Registration,
}

impl<E: Source> PollEvented<E> {
    pub fn new(mut source: E) -> Self {
        let registration = Registration::register(&mut source, context::get_executor_handle());
        PollEvented {
            io: source,
            registration: registration,
        }
    }
}
