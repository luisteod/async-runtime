use crate::runtime::io::Registration;
use mio::event::Source;

pub struct PollEvented<E: Source> {
    io: E,
    registration: Registration,
}

impl<E: Source> PollEvented<E> {
    pub fn new(source: E) -> Self {
        let registration = Registration::register(&source);
        PollEvented {
            io: source,
            registration: registration,
        }
    }
}
