use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, BufWriter};

use bytes::BytesMut;
use crate::peer::PeerPool;

use super::message::PeerMessage;


// Represents a connected peer to the network
pub struct SocketConnection {
    pub writer: BufWriter<TcpStream>,
    pub buffer: BytesMut
}
impl SocketConnection {
    pub fn new(socket: TcpStream) -> Self {
        SocketConnection {
            writer: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(1024)
        }
    }

    pub async fn handle_connection(&mut self, peers_pool: Arc<Mutex<PeerPool>>) {
        loop {
            let bytes_read = self.writer.read_buf(&mut self.buffer).await.unwrap();
            if bytes_read == 0 && self.buffer.is_empty() {
                break;
            }

            let peer_message = match self.parse_message() {
                Some(msg) => {
                    self.buffer.clear();
                    let mut peers_pool_lock = peers_pool.lock().unwrap();
                    peers_pool_lock.add_peer(msg.header);
                    msg.payload
                },
                None => continue
            };

            println!("Message received: {:?}", peer_message);
            self.buffer.clear();
        }
    }

    fn parse_message(&mut self) -> Option<PeerMessage> {
        let message: Result<PeerMessage, serde_json::Error> = serde_json::from_slice(&self.buffer[..]);
        match message {
            Ok(msg) => Some(msg),
            Err(err) => {
                println!("ERROR > Could not parse peer message -> {}", err);
                None
            }
        }

    }
}