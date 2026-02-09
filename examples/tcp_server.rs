use std::io;
use std::net::SocketAddr;

use async_runtime::net::tcp::TcpListener;

async fn async_accept(listener: &TcpListener) -> io::Result<SocketAddr> {
    let (_stream, addr) = listener.accept().await?;

    let ip = addr.ip();
    let port = addr.port();

    println!("Client IP: {ip}\nClient Port: {port}");

    // DO SOME WORK HERE

    return Ok(addr);
}

async fn serve(bind_addr: &str) {
    // Binds address
    let listener = TcpListener::bind(bind_addr).expect("Fail to bind address");

    // Create a serving loop to accept tcp connection asynchronously
    loop {
        let _ = async_accept(&listener).await;
    }
}

fn main() {
    let rt = async_runtime::build();

    rt.spawn(serve("localhost:4354"));

    rt.start();
}
