use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::resources::option::OptionResourceData;
use crate::message::types::Type;

#[derive(Clone, Debug, Default)]
pub struct OptionResourceRecordCode {
    upper_byte : i8,
    version : i8
}

impl OptionResourceRecordCode {
    pub fn upper_byte(&self) -> i8 {
        self.upper_byte
    }

    pub fn version(&self) -> i8 {
        self.version
    }

    pub fn set_upper_byte(&mut self, upper_byte: i8) {
        self.upper_byte = upper_byte;
    }

    pub fn set_version(&mut self, version: i8) {
        self.version = version;
    }
}

impl From<&mut BytesMut> for OptionResourceRecordCode {
    fn from(value: &mut BytesMut) -> Self {
        Self {
            upper_byte: value.get_i8(),
            version: value.get_i8()
        }
    }
}

impl From<OptionResourceRecordCode> for i16 {
    fn from(value: OptionResourceRecordCode) -> Self {
        0
    }
}

#[derive(Clone, Debug)]
pub struct OptionResourceRecordFlags {
    dnssec : bool, // 1 bit, https://www.rfc-editor.org/rfc/rfc3225
    zero : i16 // 15 bits
}

impl From<&mut BytesMut> for OptionResourceRecordFlags {
    fn from(value: &mut BytesMut) -> Self {
        let flags = value.get_i16();
        OptionResourceRecordFlags {
            dnssec: flags & (1 << 15) != 0,
            zero: 0
        }
    }
}

impl From<OptionResourceRecordFlags> for i16 {
    fn from(value : OptionResourceRecordFlags) -> Self {
        let mut flags : i16 = 0;
        // flags |= (value.dnssec as i16) << 15;
        flags
    }
}

// See: https://www.rfc-editor.org/rfc/rfc6891 OPT EDNS
#[derive(Builder, Clone, Debug)]
pub struct OptionResourceRecord {
    udp_payload_size: u16,
    code : OptionResourceRecordCode,
    flags: OptionResourceRecordFlags,
    data_length: u16,
    data : OptionResourceData
}

impl OptionResourceRecord {
    pub fn udp_payload_size(&self) -> u16 {
        self.udp_payload_size
    }

    pub fn code(&self) -> OptionResourceRecordCode {
        self.code.clone()
    }

    pub fn flags(&self) -> OptionResourceRecordFlags {
        self.flags.clone()
    }

    pub fn data_length(&self) -> u16 {
        self.data_length
    }

    pub fn data(&self) -> OptionResourceData {
        self.data.clone()
    }

    pub fn set_udp_payload_size(&mut self, udp_payload_size: u16) {
        self.udp_payload_size = udp_payload_size;
    }

    pub fn set_code(&mut self, code: OptionResourceRecordCode) {
        self.code = code;
    }

    pub fn set_flags(&mut self, flags: OptionResourceRecordFlags) {
        self.flags = flags;
    }

    pub fn set_data_length(&mut self, data_length: u16) {
        self.data_length = data_length;
    }

    pub fn set_data(&mut self, data: OptionResourceData) {
        self.data = data;
    }
}

impl From<&mut BytesMut> for OptionResourceRecord {
    fn from(value: &mut BytesMut) -> Self {
        let udp_payload_size = value.get_u16();
        let code = OptionResourceRecordCode::from(&mut *value);
        let flags = OptionResourceRecordFlags::from(&mut *value);
        let data_length = value.get_u16();
        let data = OptionResourceData::from(value);
        OptionResourceRecord {
            udp_payload_size,
            code,
            flags,
            data_length,
            data,
        }
    }
}
impl From<OptionResourceRecord> for BytesMut {
    fn from(value: OptionResourceRecord) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_u8(0); // Root domain
        bytes.put_u16(Type::Option as u16);
        bytes.put_u16(value.udp_payload_size);
        bytes.put_i16(i16::from(value.code.clone()));
        bytes.put_i16(i16::from(value.flags.clone()));

        let data = BytesMut::from(value.data());
        bytes.put_u16(data.len() as u16);
        bytes.put(data);
        bytes
    }
}