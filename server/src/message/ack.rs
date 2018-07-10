use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Ack {
    pub address     : Ipv4Addr,
    pub lease_time  : u32,
    pub message     : String,
}