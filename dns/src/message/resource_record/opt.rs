use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::resource_data::DnsRData;
use crate::message::types::DnsType;

// See: https://www.rfc-editor.org/rfc/rfc6891 OPT EDNS
#[derive(Builder, Clone, Debug)]
pub struct DnsOptResourceRecord {
    name: String,
    r#type: DnsType,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata : DnsRData
}

impl From<&mut BytesMut> for DnsOptResourceRecord {
    fn from(value: &mut BytesMut) -> Self {
        let name = String::from("0");
        let r#type = DnsType::OPT;
        let class = value.get_u16();
        let ttl = value.get_u32();
        let rdlength = value.get_u16();
        let rdata : DnsRData = DnsRData::from(value, &r#type);
        DnsOptResourceRecord {
            name,
            r#type,
            class,
            ttl,
            rdlength,
            rdata,
        }
    }
}