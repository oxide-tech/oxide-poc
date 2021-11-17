use std::net::Ipv4Addr;
use std::result;
use std::error::Error;
use tokio::net::TcpListener;
use p2p;

#[tokio::main]
async fn main() -> result::Result<(), Box<dyn Error>>{

    // TODO: Move this to a config or smth
    let hostname = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16 = 8080;

    // Start a new listener, thus start the TCP server
    let addr = p2p::address(hostname, port);
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Starting Oxide Node v0.1.0");
    p2p::launch_node(listener).await?;

    Ok(())
}