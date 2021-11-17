use std::net::{Ipv4Addr, IpAddr, SocketAddr};
use std::io;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::connection;


// API for starting a P2P Node
#[derive(Clone, Copy)]
pub struct Server {

    // The hostname on which the server will run
    pub hostname: Ipv4Addr,

    // The port that will be opened for listening and incoming connections
    pub port: u16
}

impl Server {
    pub fn new(hostname: Ipv4Addr, port: u16) -> Self {
        Server {hostname, port}
    }

    // Handles starting the server
    //
    // It will create a listener on the hostname and port given
    // The listener wil further accept incoming connections and process them
    // through.
    pub async fn start(self) -> io::Result<()>{

        println!("Starting server....");
        let server_addr = self.get_address();
        let listener = TcpListener::bind(server_addr).await?;
        println!("Server now listening on {}", server_addr);

        // Start listening for incoming connections loop.
        // This part should spawn new process and keep this one unlocked for
        // incoming new requests
        loop {
            let socket = self.accept_connection(&listener).await?;
            tokio::spawn(async move {
                self.handle_connection(socket).await.unwrap();
            });
        }
}

    fn get_address(&self) -> SocketAddr {
        let ip_address: IpAddr = IpAddr::V4(self.hostname);
        SocketAddr::new(ip_address, self.port)
    }

    // Accepts incoming connection
    //
    // It has a retry mechanism to check if the connection was established first
    // before returning the socket
    async fn accept_connection(&self, listener: &TcpListener) -> io::Result<TcpStream> {
        let mut retry_counter = 0;

        loop {
            match listener.accept().await {
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

    async fn handle_connection(&self, socket: TcpStream) -> io::Result<()> {
        let mut socket_connection = connection::SocketConnection::new(socket);
        let payload = socket_connection.read_payload().await?;
        match payload {
            Some(peer_message) => {
                // TODO: Process message in peers
                println!("{:?}", peer_message);
                Ok(())
            },
            None => {
                // TODO: process error reading message
                println!("No payload");
                panic!("fck")
            }
        }
    }
}

