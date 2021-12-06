use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use serde::{Serialize, Deserialize};

// This is the header that comes through with each message.
// The peer that sends a message through the network should also
// have attached this header.
//
// It is also used to store a peer inside a peer pool.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub hostname: Ipv4Addr,
    pub port: u16
}
impl Header {
    pub fn new(peer: String) -> Self {
        let peer_data: Vec<&str> = peer.split(":").collect();
        Header {
            hostname: Ipv4Addr::from_str(peer_data[0]).unwrap(),
            port: u16::from_str(peer_data[1]).unwrap()
        }
    }
    pub fn address(&self) -> SocketAddr {
        let ip_address: IpAddr = IpAddr::V4(self.hostname);
        SocketAddr::new(ip_address, self.port)
    }
}

// Contains all neighbour nodes
pub struct PeerPool {
    pub peers: HashMap<String, Header>
}
impl PeerPool {

    pub fn new() -> Self {
        PeerPool { peers: HashMap::new() }
    }

    pub fn add_peer(&mut self, header: Header) {
        println!("INFO > Adding new connection to peer pool. peer: {:?}", header);
        self.peers.entry(header.hostname.to_string()).or_insert(header);
    }

    pub fn remove_peer(&mut self, hostname: Ipv4Addr) {
        self.peers.remove(&hostname.to_string());
    }

    pub fn get_alive_peers(&self, count: usize) -> Vec<&Header> {
        let mut requested_peers: Vec<&Header> = Vec::new();
        let counter: usize = 0;

        for (_, peer) in &self.peers {
            if counter >= count {
                return requested_peers;
            }
            requested_peers.push(peer)
        }

        requested_peers
    }

}

