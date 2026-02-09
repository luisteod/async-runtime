use crate::io::PollEvented;
use futures::io;
use mio::Interest;
use std::net::SocketAddr;

pub struct TcpListener {
    io: PollEvented<mio::net::TcpListener>,
}

impl TcpListener {
    fn new(listener: mio::net::TcpListener) -> Self {
        let poll_evented = PollEvented::new(listener, Interest::READABLE);
        TcpListener { io: poll_evented }
    }

    /// Not async yet
    pub fn bind(addr: &str) -> io::Result<Self> {
        // Solve DNS if needed. Could be async in future
        let addrs = std::net::ToSocketAddrs::to_socket_addrs(&addr)?;

        let mut last_err = None;

        let try_bind = |addr| {
            let listener = mio::net::TcpListener::bind(addr)?;
            Ok(TcpListener::new(listener))
        };

        for addr in addrs {
            match try_bind(addr) {
                Ok(listener) => return Ok(listener),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve to any address",
            )
        }))
    }

    /// Not finished yet. Currently returning the no async version of TcpStream from mio.
    /// In the future this method must return a custom awaitable TcpStream.
    pub async fn accept(&self) -> io::Result<(mio::net::TcpStream, SocketAddr)> {
        let (mio, addr) = self
            .io
            .registration()
            .async_io(Interest::READABLE, || self.io.accept())
            .await?;

        Ok((mio, addr))
    }
}
