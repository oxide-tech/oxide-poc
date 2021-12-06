use std::env;
use std::process;

use tokio::net::{TcpListener};

use node;

#[tokio::main]
async fn main() {
     println!("Starting Oxide Node v0.2.0");

     let node_address: String = match env::var("NODE_HOST") {
          Ok(val) => val,
          Err(_) => {
               println!("ERROR > Could not find NODE_HOST in environment!!");
               process::exit(0x0100);
          }
     };
     let node_peers: String = match env::var("NODE_PEERS") {
          Ok(val) => val,
          Err(_) => {
               println!("WARN > No peers provided");
               String::new()
          }
     };
     let peers = node_peers.split("::").filter(|&x| !x.is_empty()).collect();

     let listener = TcpListener::bind(node_address).await.unwrap();
     let mut server = node::server::Node::new(listener, peers);

     server.run().await;
}