mod hinfo;
mod opt;

use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::types::DnsType;
use crate::message::resource_data::hinfo::DnsHInfoRData;
use crate::message::resource_data::opt::DnsOptRData;

type DnsDomainName = String;

#[derive(Clone, Debug)]
pub enum DnsRData {
    CNAME(DnsDomainName),
    HINFO(DnsHInfoRData),
    MB(String),
    MD(String),
    MF(String),
    MG(String),
    MINFO(String,String),
    MR(String),
    MX(u16,String),
    NULL(Vec<u8>),
    NS(String),
    PTR(String),
    SOA(String,String,u32,u32,u32,u32,u32),
    TXT(String),
    RDATA(u32),
    WKS(u32,u8,Vec<u8>),
    OPT(DnsOptRData)
}

impl DnsRData {
    pub fn from(bytes: &mut BytesMut, r#type: &DnsType) -> Self {
        match r#type {
            DnsType::CNAME => DnsRData::CNAME(bytes.get_u16().to_string()),
            DnsType::HINFO => DnsRData::HINFO(DnsHInfoRData::from(&mut *bytes)),
            DnsType::OPT => DnsRData::OPT(DnsOptRData::from(bytes)),
            _ => panic!("Unknown DNS RDATA"),
        }
    }
}