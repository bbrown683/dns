use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use faster_hex::hex_string;
use crate::message::records::{ResourceRecord, ResourceRecordType};

#[derive(Debug, Clone)]
pub enum OptionRecordCode {
    Cookie = 10,
}

impl From<u16> for OptionRecordCode {
    fn from(value: u16) -> Self {
        match value {
            _ => OptionRecordCode::Cookie,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OptionRecordDataOption {
    Cookie(OptionCookie)
}

impl OptionRecordDataOption {
    fn from(value: &mut Vec<u8>, r#type : &OptionRecordCode) -> Self {
        if matches!(r#type, OptionRecordCode::Cookie) {
            OptionRecordDataOption::Cookie(OptionCookie::from(value))
        } else {
            panic!("unexpected resource record option")
        }
    }
}

// See: https://www.rfc-editor.org/rfc/rfc7873
#[derive(Builder, Clone, Debug, Default)]
pub struct OptionCookie {
    client: String,
    server: String,
}

impl From<&mut Vec<u8>> for OptionCookie {
    fn from(value: &mut Vec<u8>) -> Self {
        // Client cookie is a fixed 8 bytes, Server cookie is 8-32 bytes.
        let (client_cookie_bytes, server_cookie_bytes) = value.split_at(8);
        let client = hex_string(client_cookie_bytes);
        let mut server = String::new();
        if !server_cookie_bytes.is_empty() {
            server = hex_string(server_cookie_bytes);
        }
        OptionCookie {
            client,
            server
        }
    }
}

#[derive(Builder, Clone, Debug)]
pub struct OptionResourceData {
    code : OptionRecordCode,
    length : u16,
    data : OptionRecordDataOption
}

impl From<&mut BytesMut> for OptionResourceData {
    fn from(value: &mut BytesMut) -> Self {
        let code = OptionRecordCode::from(value.get_u16());
        let length = value.get_u16();

        let mut data_bytes = vec![0; length as usize];
        value.copy_to_slice(&mut data_bytes);
        let data = OptionRecordDataOption::from(&mut data_bytes, &code);

        OptionResourceData {
            code,
            length,
            data,
        }
    }
}

impl From<OptionResourceData> for BytesMut {
    fn from(value: OptionResourceData) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_u16(value.code as u16);
        bytes.put_u16(value.length);
        match value.data {
            OptionRecordDataOption::Cookie(cookie) => {
                bytes.put(cookie.client.as_bytes());
                if !cookie.server.is_empty() {
                    bytes.put(cookie.server.as_bytes());
                }
            }
        }
        bytes
    }
}