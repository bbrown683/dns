use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use faster_hex::{hex_string, hex_decode};

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

impl OptionCookie {
    pub fn client(&self) -> &str {
        &self.client
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn set_client(&mut self, client: String) {
        self.client = client;
    }

    pub fn set_server(&mut self, server: String) {
        self.server = server;
    }
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
    data : OptionRecordDataOption
}

impl OptionResourceData {
    pub fn code(&self) -> OptionRecordCode {
        self.code.clone()
    }


    pub fn data(&self) -> OptionRecordDataOption {
        self.data.clone()
    }

    pub fn set_code(&mut self, code: OptionRecordCode) {
        self.code = code;
    }

    pub fn set_data(&mut self, data: OptionRecordDataOption) {
        self.data = data;
    }
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
            data,
        }
    }
}

impl From<OptionResourceData> for BytesMut {
    fn from(value: OptionResourceData) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_u16(value.code as u16);
        match value.data {
            OptionRecordDataOption::Cookie(cookie) => {
                let mut client = vec![0; cookie.client.len() / 2];
                hex_decode(&cookie.client.as_bytes(), &mut client).expect("Failed to decode hex.");

                let mut server = vec![0; cookie.server.len() / 2];
                hex_decode(&cookie.server.as_bytes(), &mut server).expect("Failed to decode hex.");

                let length = client.len() + server.len();
                bytes.put_u16(length as u16);
                bytes.put(client.as_slice());
                bytes.put(server.as_slice());
            }
        }
        bytes
    }
}