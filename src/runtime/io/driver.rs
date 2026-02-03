// Hash Three
// Deve conter um hashing de quando uma fonte de io é criada. Serve pra armazenar uma estrutura tipo ScheduledIo
// que conterá uma lista de wakers

// Trava no poll e dispacha os eventos de acordo com o token recebido do evento. Procura no hashing a fonte em
// específico e chama o wake() para ela.
// pub fn turn

use super::scheduled_io::ScheduledIo;
use mio::{Events, Poll, Token, event::Source, Interest};
use std::collections::HashMap;

const MAX_EVENTS_CAPACITY: usize = 1024;
const DEFAULT_IO_INTERESTS: Interest = Interest::READABLE.add(Interest::WRITABLE);

pub struct Driver {
    poll: Poll,
    scheduled_io_map: HashMap<Token, ScheduledIo>,
}

impl Driver {
    pub fn new() -> Self {
        let poll = Poll::new().expect("Error creating Poll instance");
        let scheduled_io_map = HashMap::new();

        Driver {
            poll,
            scheduled_io_map,
        }
    }

    pub fn add_source<E>(&mut self, source: &mut E)
    where
        E: Source,
    {
        let scheduled_io = ScheduledIo::default();
        let token = scheduled_io.token();

        self.scheduled_io_map.insert(token, scheduled_io);
        self.poll.registry().register(source, token, DEFAULT_IO_INTERESTS);
    }

    pub fn park(&mut self) {
        let mut events = Events::with_capacity(MAX_EVENTS_CAPACITY);
        self.poll
            .poll(&mut events, None)
            .expect("Error polling IO events");

        for event in &events {
            let token = event.token();

            let io = self.scheduled_io_map.get(&token);
            io.expect("No waker provided").wake();
        }
    }
}
