use std::sync::Mutex;
use std::sync::Arc;
use std::net::{Ipv4Addr, IpAddr, SocketAddr};

use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::connection::SocketConnection;
use crate::protocol::pool::PeersPool;
use crate::{ServerResult};


// API for starting a P2P Node
pub struct Server {
    pub listener: TcpListener,
    pub peers_connections: Arc<Mutex<PeersPool>>
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server {
            listener,
            peers_connections: Arc::new(Mutex::new(PeersPool::new()))
        }
    }

    async fn start(&mut self) -> ServerResult<()> {
        loop {
            let socket = self.accept_connection().await?;
            let mut connection = SocketConnection::new(socket);

            let peers = self.peers_connections.clone();
            tokio::spawn(async move {
                if let Err(err) = connection.handle_connection(&peers).await {
                    println!("Connection process error occured {}", err);
                }
            });
        }
    }

    // Accepts incoming connection
    //
    // It has a retry mechanism to check if the connection was established first
    // before returning the socket
    async fn accept_connection(&self) -> ServerResult<TcpStream> {
        let mut retry_counter = 0;

        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => {
                    println!("New incoming connection established");
                    return Ok(socket)
                }
                Err(err) => {
                    println!("Could not establish a connection. Retrying {}", retry_counter);
                    if retry_counter > 3 {
                        return Err(err.into());
                    }
                }
            };
            retry_counter += 1;
        }
    }
}

pub async fn launch_node(listener: TcpListener) -> ServerResult<()> {
    let mut server = Server::new(listener);
    println!("Server now listening on {:?}", server.listener.local_addr().unwrap());

    tokio::select! {
        resp = server.start() => {
            if let Err(err) = resp {
                println!("Failed to accept a connection");
            }
        }
    }

    Ok(())
}