use std::io;
use std::net::SocketAddr;

use async_runtime::net::tcp::TcpListener;

/// Asynchrounous serve TCP clients.

async fn async_accept(listener: &TcpListener) -> io::Result<SocketAddr> {
    let (_stream, addr) = listener.accept().await?;

    let ip = addr.ip();
    let port = addr.port();

    println!("Client IP: {ip}\nClient Port: {port}");

    // ** DO SOME WORK HERE **

    return Ok(addr);
}

async fn serve(bind_addr: &str) {
    // Binds address
    let listener = TcpListener::bind(bind_addr).expect("Fail to bind address");

    // Create a serving loop to accept tcp connections asynchronously
    loop {
        let _ = async_accept(&listener).await;
    }
}

fn main() {
    // Builds the async runtime
    let rt = async_runtime::build();

    // Spawn the server entry-point
    rt.spawn(serve("localhost:4354"));

    // Start the runtime
    rt.start();
}
