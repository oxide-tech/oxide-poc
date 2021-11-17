use p2p::server;
use std::net;

#[tokio::main]
async fn main(){
    let hostname = net::Ipv4Addr::new(0, 0, 0, 0);
    let port: u16 = 8080;

    let server = server::Server::new(hostname, port);
    server.start().await.unwrap();
}