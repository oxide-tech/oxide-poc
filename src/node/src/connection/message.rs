use serde::Serialize;
use serde::Deserialize;

use crate::peer::Header;

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerMessage {

    // Can be also viewed as the header of the request
    pub header: Header,

    // The cargo of a message
    pub payload: String
}