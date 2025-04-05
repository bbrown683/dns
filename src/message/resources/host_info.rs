use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;

// https://www.rfc-editor.org/rfc/rfc1010
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

impl From<HostInfoResourceData> for BytesMut {
    fn from(value: HostInfoResourceData) -> Self {
        let mut bytes = BytesMut::new();
        bytes
    }
}