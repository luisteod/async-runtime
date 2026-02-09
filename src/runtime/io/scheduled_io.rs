use std::pin::Pin;
use std::sync::{Arc, Mutex, atomic::AtomicUsize, atomic::Ordering};
use std::task::{Context, Poll, Waker};

use mio::event::Event;
use mio::{Interest, Token};

/// Defining custom interests bitmask since [`mio::Interest`] don't expose it.
const READABLE: usize = 0b01;
const WRITABLE: usize = 0b10;

/// Represents an IO source that was added into the [`crate::runtime::io::IoDriver`].
/// This entry is placed into a HashMap inside the driver using it's generated token.
/// When the driver identify an IO event, it calls [`ScheduledIo::wake`] of the
/// appropriate entry.
pub struct ScheduledIo {
    waker: Mutex<Option<Waker>>,
    interest: AtomicUsize,
    last_event: Mutex<Option<Event>>,
}

pub struct IoEventFuture {
    // We hold a clone of the Arc so the state stays alive while we await
    io: Arc<ScheduledIo>,
}

impl ScheduledIo {
    pub fn default() -> Self {
        ScheduledIo {
            waker: Mutex::new(None),
            interest: AtomicUsize::new(0), // 0 is no interest
            last_event: Mutex::new(None),
        }
    }

    // Handle for waking this future
    pub fn wake(&self) {
        // Lock the mutex briefly to get the waker
        if let Some(waker) = self.waker.lock().unwrap().as_ref() {
            waker.wake_by_ref();
        }
    }

    pub fn set_event(&self, event: Event) {
        // Mutex::lock() replaces RefCell::borrow_mut()
        let mut slot = self.last_event.lock().unwrap();
        *slot = Some(event);
    }

    pub fn io_future(self: &Arc<Self>, interest: Interest) -> IoEventFuture {
        let mut bits = 0;

        if interest.is_readable() {
            bits |= READABLE;
        }
        if interest.is_writable() {
            bits |= WRITABLE;
        }

        self.interest.store(bits, Ordering::Release);

        IoEventFuture { io: self.clone() }
    }

    /// This is much safer now that Arc is being used. Since Arc allocates on the heap,
    /// the address of ScheduledIo will never change as long as at least one Arc exists.
    /// This is ssentially using the heap address as a unique ID for the OS to send back it back.
    pub fn token(&self) -> Token {
        let ptr = self as *const ScheduledIo;
        Token(ptr.addr())
    }
}

impl Future for IoEventFuture {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Reach into the shared state to check for readiness
        {
            let mut event_slot = self.io.last_event.lock().unwrap();
            if let Some(event) = event_slot.take() {
                return Poll::Ready(event);
            }
        }

        // If not ready, we register the waker in the shared state
        let mut waker_slot = self.io.waker.lock().unwrap();
        *waker_slot = Some(cx.waker().clone());

        Poll::Pending
    }
}
