use std::collections::HashMap;
use std::net::Ipv4Addr;

use crate::payload::Meta;

// Contains all neighbour nodes.
pub struct PeersPool {
    pub peers: HashMap<Ipv4Addr, Meta>
}

impl PeersPool {
    pub fn new() -> Self {
        PeersPool { peers: HashMap::new() }
    }

    pub fn add_peer(&mut self, hostname: Ipv4Addr, meta: Meta) {
        println!("Adding new connection to pool {:?}", meta);
        self.peers.entry(hostname).or_insert(meta);
    }

    pub fn remove_peer(&mut self, hostname: Ipv4Addr) {
        self.peers.remove(&hostname);
    }

    pub fn get_alive_peers(&self, count: usize) -> Vec<&Meta> {
        let mut requested_peers: Vec<&Meta> = Vec::new();
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