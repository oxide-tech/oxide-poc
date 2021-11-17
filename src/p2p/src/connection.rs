use std::{result, io};
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use bytes::BytesMut;

use serde_json;

use crate::{
    payload::PeerMessage,
    protocol::pool::PeersPool
};

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

    fn parse_message(&self) -> Option<PeerMessage> {
        let message: result::Result<PeerMessage, serde_json::Error> = serde_json::from_slice(&self.buffer[..]);
        match message {
            Ok(payload) => Some(payload),
            Err(err) => {
                println!("Incorrect message received {}", err);
                None
            }
        }
    }

    fn pack_message(&self, payload: &PeerMessage) -> result::Result<Vec<u8>, io::Error> {
        let payload_pack = serde_json::to_vec(payload);
        match payload_pack {
            Ok(package) => Ok(package),
            Err(err) => Err(err.into())
        }
    }

    pub async fn read_payload(&mut self) -> result::Result<Option<PeerMessage>, io::Error>{
        let size = self.writer.read_buf(&mut self.buffer).await?;
        if self.buffer.is_empty() && size == 0{
            Ok(None)
        } else {
            match self.parse_message() {
                Some(payload) => {
                    self.buffer.clear();
                    Ok(Some(payload))
                },
                None => {
                    self.buffer.clear();
                    Ok(None)
                }
            }
        }
    }

    pub async fn write_payload(&mut self, payload: &PeerMessage) -> io::Result<()> {
        let package = self.pack_message(payload)?;
        self.writer.write_all(&package[..]).await?;

        println!("Sending package to node");

        Ok(())
    }

    pub async fn handle_connection(
        &mut self,
        peers_connections: &Arc<Mutex<PeersPool>>
    ) -> io::Result<()> {

        loop {
            let package = self.read_payload().await?;
            match package {
                
            }
        }
    }

}