use crate::io::PollEvented;

struct TcpListener {
    io: PollEvented<mio::net::TcpListener>,
}
