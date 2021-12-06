use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::peer::PeerPool;
use crate::peer::Header;
use crate::connection::PeerMessage;
use crate::ServerResult;
use crate::connection::socket::SocketConnection;


// The Oxide Node server.
//
// Contains the listener, the peers pool and the methods to run it.
// When the node is started it will start to listen for incoming connections and
// it will broadcast information when blockchain state changes
pub struct Node {
    pub header: Header,
    pub listener: TcpListener,
    pub peers_pool: Arc<Mutex<PeerPool>>
}
impl Node {
    pub fn new(listener: TcpListener, node_peers: Vec<&str>) -> Self {
        let node_address = listener.local_addr().unwrap().to_string();
        let node = Node {
            listener,
            peers_pool: Arc::new(Mutex::new(PeerPool::new())),
            header: Header::new(node_address)
        };
        for peer in node_peers.iter() {
            let mut pool_data = node.peers_pool.lock().unwrap();
            let new_peer_header = Header::new(peer.to_string());
            pool_data.add_peer(new_peer_header);
        }
        node

    }

    // The node already has a listening TcpListener active, this method will start with
    // accepting new connections.
    pub async fn run(&mut self) {
        println!("INFO > Oxide Node now listening on {}", self.listener.local_addr().unwrap());

        // As the node will start it has to broadcast its presence to the network peers.
        self.broadcast_to_peers().await.unwrap();

        tokio::select! {
            resp = self.accept_connection() => {
                if let Err(err) = resp {
                    println!("ERROR: It failed to handle connection. ERR: {}", err);
                }
            }
        }
    }

    // Listens and accepts new incoming connections. This creates a new socket connection which
    // should be then passed to a connection handler.
    async fn accept_connection(&mut self) -> ServerResult<()>{
        loop {
            let (socket, _address) = self.listener.accept().await.unwrap();
            let peers_pool = Arc::clone(&self.peers_pool);

            tokio::spawn(async move {
                println!("INFO > New socket connected -> {}", _address);
                let mut socket_connection = SocketConnection::new(socket);
                socket_connection.handle_connection(peers_pool).await;
                println!("INFO > Socket disconnected -> {}", _address);
            });
        }
    }

    async fn broadcast_to_peers(&mut self) -> ServerResult<()> {
        let peers_data = self.peers_pool.lock().unwrap();
        let alive_peers = peers_data.get_alive_peers(3);
        for peer in alive_peers.iter() {
            match TcpStream::connect(peer.address()).await {
                Ok(mut stream) => {
                    let peer_message = PeerMessage {
                        header: self.header.clone(),
                        payload: String::from("Hello from a new node")
                    };
                    let (_reader, mut writer) = stream.split();

                    let msg = serde_json::to_vec(&peer_message).unwrap();

                    writer.write_all(msg.as_slice()).await.unwrap();
                    println!("Ping sent");
                },
                Err(_) => {
                    continue
                }
            }
        }
        Ok(())
    }
}
