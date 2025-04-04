use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use faster_hex::hex_string;
use crate::message::resource_record::{DnsResourceRecord, DnsResourceRecordExtension};

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum DnsOptRCode {
    COOKIE = 10,
}

impl From<u16> for DnsOptRCode {
    fn from(value: u16) -> Self {
        match value {
            _ => DnsOptRCode::COOKIE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DnsOptRDataOption {
    COOKIE(DnsOptCookie)
}

impl DnsOptRDataOption {
    fn from(value: &mut Vec<u8>, r#type : &DnsOptRCode) -> Self {
        if matches!(r#type, DnsOptRCode::COOKIE) {
            DnsOptRDataOption::COOKIE(DnsOptCookie::from(value))
        } else {
            panic!("unexpected resource record option")
        }
    }
}

// See: https://www.rfc-editor.org/rfc/rfc7873
#[derive(Builder, Clone, Debug)]
pub struct DnsOptCookie {
    client_cookie: String,
    server_cookie: String,
}

impl From<&mut Vec<u8>> for DnsOptCookie {
    fn from(value: &mut Vec<u8>) -> Self {
        // Client cookie is a fixed 8 bytes, Server cookie is 8-32 bytes.
        let (client_cookie_bytes, server_cookie_bytes) = value.split_at(8);
        let client_cookie = hex_string(client_cookie_bytes);
        let mut server_cookie = String::new();
        if !server_cookie_bytes.is_empty() {
            server_cookie = hex_string(server_cookie_bytes);
        }
        println!("Cookie: {}", client_cookie);
        DnsOptCookie {
            client_cookie,
            server_cookie
        }
    }
}

#[derive(Builder, Clone, Debug)]
pub struct DnsOptRData {
    code : DnsOptRCode,
    length : u16,
    data : DnsOptRDataOption
}

impl From<&mut BytesMut> for DnsOptRData {
    fn from(value: &mut BytesMut) -> Self {
        let code = DnsOptRCode::from(value.get_u16());
        let length = value.get_u16();

        let mut data_bytes = vec![0; length as usize];
        value.copy_to_slice(&mut data_bytes);
        let data = DnsOptRDataOption::from(&mut data_bytes, &code);

        DnsOptRData {
            code,
            length,
            data,
        }
    }
}