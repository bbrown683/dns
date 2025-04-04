use std::net::Ipv4Addr;
use derive_builder::Builder;

// https://www.rfc-editor.org/rfc/rfc1010
#[derive(Clone, Debug)]
pub enum Protocol {
    TCP = 6,
    UDP = 17
}

#[derive(Builder, Clone, Debug)]
pub struct WellKnownServiceResourceData {
    address: Ipv4Addr,
    protocol: Protocol,
    bitmap: Vec<u8>,
}