use std::net::TcpStream;
use std::io::{Read, Write};
use std::net;

use serde_json;

use p2p::payload::Meta;
use p2p::payload::Ping;
use p2p::payload::PeerMessage;

fn main() {
    match TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            let hostname = net::Ipv4Addr::new(0, 0, 0, 0);
            let port: u16 = 8080;
            println!("Connected to the oxide node");
            let meta = Meta {hostname: hostname, port: port};
            let ping = Ping { msg: String::from("Hello from the client")};
            let payload = PeerMessage {
                meta: meta,
                payload: ping
            };
            let sera = serde_json::to_vec(&payload).unwrap();
            stream.write(&sera[..]).unwrap();
            println!("message sent");
        },
        Err(er) => {
            println!("Failed to connect to oxide")
        }
    }
}