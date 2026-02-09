use async_runtime::net::tcp::TcpListener;
use futures::io;
use std::net::SocketAddr;

async fn async_tcp(addr: &str) -> io::Result<SocketAddr> {
    let listener = TcpListener::bind(addr)?;

    let (_stream, addr) = listener.accept().await?;

    let ip = addr.ip();
    let port = addr.port();

    println!("Client IP: {ip}\nClient Port: {port}");

    return Ok(addr);
}

async fn async_main() {
    let bind_addr = "localhost:4354";
    let _ = async_tcp(bind_addr).await;
}

fn main() {
    let rt = async_runtime::build();

    rt.spawn(async_main());

    rt.start();
}
