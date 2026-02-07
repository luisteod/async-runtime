use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use mio::event::Event;
use mio::{Interest, Token};
use std::cell::RefCell;

/// Represents an IO source that was added into the [`crate::runtime::io::IoDriver`].
/// This entry is placed into a HashMap inside the driver using it's generated token.
/// When the driver identify an IO event, it calls [`ScheduledIo::wake`] of the
/// appropriate entry.

pub struct ScheduledIo {
    waker: Option<Waker>,
    interest: Option<Interest>,
    last_event: RefCell<Option<Event>>,
}

impl ScheduledIo {
    pub fn default() -> Self {
        ScheduledIo {
            waker: None,
            interest: None,
            last_event: RefCell::new(None),
        }
    }

    // Handle for waking this future
    pub fn wake(&self) {
        self.waker
            .as_ref()
            .expect("Waker not provided")
            .wake_by_ref();
    }

    pub fn set_event(&self, event: Event) {
        self.last_event.replace(Some(event));
    }

    pub fn io_future(mut self, interest: Interest) -> Self {
        self.interest = Some(interest);
        self
    }

    pub fn token(&self) -> Token {
        let ptr = self as *const ScheduledIo;
        Token(ptr.addr())
    }
}

impl Future for ScheduledIo {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let io = self.get_mut();

        // Handles cleaning of the created RefMut guard
        {
            let mut event_slot = io.last_event.borrow_mut();

            if let Some(event) = event_slot.take() {
                return Poll::Ready(event);
            }
        }

        io.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}
