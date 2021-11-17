pub mod server;
mod connection;
pub mod payload;


#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use crate::*;

    #[test]
    fn create_new_server(){
        let addr = Ipv4Addr::new(0,0,0,0);
        let port: u16 = 8080;

        let test_server = server::Server::new(addr, port);

        assert_eq!(test_server.hostname, Ipv4Addr::new(0,0,0,0));
        assert_eq!(test_server.port, 8080)
    }

}
