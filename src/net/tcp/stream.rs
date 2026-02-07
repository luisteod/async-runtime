use crate::io::PollEvented;

pub struct TcpStream {
    io: PollEvented<mio::net::TcpStream>,
}
