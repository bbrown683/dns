use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::resources::option::OptionResourceData;
use crate::message::types::Type;

#[derive(Clone, Debug, Default)]
pub struct OptionResourceRecordCode {
    upper_byte : i8,
    version : i8
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
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct OptionResourceRecordFlags {
    dnssec_ok : bool, // https://www.rfc-editor.org/rfc/rfc3225
    zero : i16
}

impl From<&mut BytesMut> for OptionResourceRecordFlags {
    fn from(value: &mut BytesMut) -> Self {
        let flags = value.get_i16();
        OptionResourceRecordFlags {
            dnssec_ok: ((flags >> 15) & 1) != 0,
            zero: 0
        }
    }
}

impl From<OptionResourceRecordFlags> for i16 {
    fn from(value : OptionResourceRecordFlags) -> Self {
        let mut flags : i16 = 0;
        flags ^= (value.dnssec_ok as i16) << 15;
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
        bytes.put_u16(0); // Root domain
        bytes.put_u16(Type::Option as u16);
        bytes.put_u16(value.udp_payload_size);
        bytes.put_i16(value.flags.into());
        bytes.put_u16(value.data_length);
        // bytes.put(OptRecordData::from(value.record_data));
        bytes
    }
}