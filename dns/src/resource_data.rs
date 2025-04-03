use std::collections::HashMap;
use bytes::{Buf, BufMut, BytesMut};
use crate::types::DnsType;

#[derive(Debug)]
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

pub enum DnsOptRCode {
    COOKIE = 10,
}

// See: https://www.rfc-editor.org/rfc/rfc7873
pub struct DnsOptCookie {

}

#[derive(Debug)]
pub struct DnsOptRData {
    code : u16,
    length : u16,
    data : HashMap<String,String>
}

impl From<&mut BytesMut> for DnsOptRData {
    fn from(value: &mut BytesMut) -> Self {
        let code = value.get_u16();
        let length = value.get_u16();
        let mut data = HashMap::new();
        DnsOptRData {
            code,
            length,
            data,
        }
    }
}

#[derive(Debug)]
pub enum DnsRData {
    CNAME(String),
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
            DnsType::CNAME => {
                let cname = bytes.get_u16().to_string();
                DnsRData::CNAME(cname)
            }
            DnsType::HINFO => {
                DnsRData::HINFO(DnsHInfoRData::from(&mut *bytes))
            },
            DnsType::OPT => {
                DnsRData::OPT(DnsOptRData::from(bytes))
            },
            _ => panic!("Unknown DNS RDATA"),
        }
    }
}