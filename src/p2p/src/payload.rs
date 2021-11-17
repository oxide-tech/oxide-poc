use std::net::Ipv4Addr;
use serde::Serialize;
use serde::Deserialize;

use std::fmt::Debug;

// #### HEADER ####


// The meta object represent the peer identity that is sending a message
// through the network.
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub hostname: Ipv4Addr,
    pub port: u16
}

// #### MESSAGES TYPES ####

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub msg: String
}

// #### SEND MESSAGES ####

// Represent the message object that will be serialized and sent through the
// network.
#[derive(Serialize, Deserialize, Debug)]
pub struct PeerMessage {

    // Can be also viewed as the header of the request
    pub meta: Meta,

    // The cargo of a message
    pub payload: Ping
}