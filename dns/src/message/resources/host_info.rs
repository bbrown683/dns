use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;

#[derive(Builder, Clone, Debug, Default)]
pub struct HostInfoResourceData {
    cpu : String,
    os : String
}

impl From<&mut BytesMut> for HostInfoResourceData {
    fn from(value: &mut BytesMut) -> Self {
        let cpu = String::new();
        let os = String::new();
        HostInfoResourceData { cpu, os }
    }
}