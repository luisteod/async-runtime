use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use mio::Token;

/// Represents an IO source that was added into the [`crate::runtime::io::IoDriver`].
/// This entry is placed into a HashMap inside the driver using it's generated token.
/// When the driver identify an IO event, it calls [`ScheduledIo::wake`] of the
/// appropriate entry.

pub struct ScheduledIo {
    waker: Option<Waker>,
    is_ready: bool,
}

impl ScheduledIo {
    pub fn default() -> Self {
        ScheduledIo {
            waker: None,
            is_ready: false,
        }
    }

    // Handle for waking this future
    pub fn wake(&self) {
        self.waker
            .as_ref()
            .expect("Waker not provided")
            .wake_by_ref();
    }

    pub fn token(&self) -> Token {
        let ptr = self as *const ScheduledIo;
        Token(ptr.addr())
    }
}

impl Future for ScheduledIo {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let io = unsafe { self.get_unchecked_mut() };
        if io.is_ready {
            return Poll::Ready(());
        }
        io.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}
