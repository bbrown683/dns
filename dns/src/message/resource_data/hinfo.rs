use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;

#[derive(Builder, Clone, Debug)]
pub struct DnsHInfoRData {
    cpu : String,
    os : String
}

impl From<&mut BytesMut> for DnsHInfoRData {
    fn from(value: &mut BytesMut) -> Self {
        let cpu = String::new();
        let os = String::new();
        DnsHInfoRData { cpu, os }
    }
}