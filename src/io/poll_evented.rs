use crate::runtime::{context, io::Registration};
use mio::event::Source;
use std::ops::Deref;

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

    pub fn registration(&self) -> &Registration {
        return &self.registration
    }
}

impl<E: Source> Deref for PollEvented<E> {
    type Target = E;

    fn deref(&self) -> &E {
        &self.io
    }
}
