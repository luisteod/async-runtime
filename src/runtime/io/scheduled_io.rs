use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

use mio::event::Event;
use mio::{Interest, Token};
use std::cell::{Cell, RefCell};

/// Represents an IO source that was added into the [`crate::runtime::io::IoDriver`].
/// This entry is placed into a HashMap inside the driver using it's generated token.
/// When the driver identify an IO event, it calls [`ScheduledIo::wake`] of the
/// appropriate entry.

pub struct ScheduledIo {
    waker: RefCell<Option<Waker>>,
    interest: Cell<Option<Interest>>,
    last_event: RefCell<Option<Event>>,
}

pub struct IoEventFuture {
    // We hold a clone of the Rc so the state stays alive while we await
    io: Rc<ScheduledIo>,
}

impl ScheduledIo {
    pub fn default() -> Self {
        ScheduledIo {
            waker: RefCell::new(None),
            interest: Cell::new(None),
            last_event: RefCell::new(None),
        }
    }

    // Handle for waking this future
    pub fn wake(&self) {
        self.waker
            .borrow()
            .as_ref()
            .expect("Waker not provided")
            .wake_by_ref();
    }

    pub fn set_event(&self, event: Event) {
        self.last_event.replace(Some(event));
    }

    pub fn io_future(self: &Rc<Self>, interest: Interest) -> IoEventFuture {
        self.interest.set(Some(interest));

        IoEventFuture { io: self.clone() }
    }

    pub fn token(&self) -> Token {
        let ptr = self as *const ScheduledIo;
        Token(ptr.addr())
    }
}

impl Future for IoEventFuture {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // We reach into the shared state to check for readiness
        let mut event_slot = self.io.last_event.borrow_mut();

        if let Some(event) = event_slot.take() {
            return Poll::Ready(event);
        }

        // If not ready, we register the waker in the shared state
        // You'll need to make 'waker' a RefCell inside ScheduledIo too!
        *self.io.waker.borrow_mut() = Some(cx.waker().clone());

        Poll::Pending
    }
}
