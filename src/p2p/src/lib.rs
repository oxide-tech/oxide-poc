use std::net::{Ipv4Addr, IpAddr, SocketAddr};
use std::result;
use std::error;

pub mod server;
pub use server::launch_node;
pub mod payload;

mod connection;
mod protocol;

pub type ServerError = Box<dyn error::Error>;
pub type ServerResult<T> = result::Result<T, ServerError>;

pub fn address(hostname: Ipv4Addr, port: u16) -> SocketAddr {
    let ip_address: IpAddr = IpAddr::V4(hostname);
    SocketAddr::new(ip_address, port)
}
