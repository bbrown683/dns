mod opt;

use bytes::{Buf, BytesMut};
use derive_builder::Builder;
use crate::message::classes::DnsClass;
use crate::message::resource_data::DnsRData;
use crate::message::traits::RepeatFrom;
use crate::message::types::DnsType;
use crate::message::resource_record::opt::DnsOptResourceRecord;

#[derive(Builder, Clone, Debug)]
pub struct DnsResourceRecord {
    name: String,
    r#type: DnsType,
    class: DnsClass,
    ttl: u32,
    rdlength: u16,
    rdata : DnsRData
}

impl DnsResourceRecord {
    fn from(value: &mut BytesMut, name : String, r#type : DnsType) -> Self {
        let class = DnsClass::from(value.get_u16());
        let ttl = value.get_u32();
        let rdlength = value.get_u16();
        let rdata : DnsRData = DnsRData::from(value, &r#type);
        DnsResourceRecord {
            name,
            r#type,
            class,
            ttl,
            rdlength,
            rdata,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DnsResourceRecordExtension {
    RR(DnsResourceRecord),
    OPT(DnsOptResourceRecord),
}

impl From<&mut BytesMut> for DnsResourceRecordExtension {
    fn from(value: &mut BytesMut) -> Self {
        let mut name = String::new();
        let mut length = value.get_u8();
        while length != 0 {
            let mut label_bytes = vec![0; length as usize];
            value.copy_to_slice(&mut label_bytes);

            let mut label = String::new();
            label.push_str(std::str::from_utf8(&label_bytes).unwrap());
            name.push_str(&label);
            name.push('.');

            length = value.get_u8();
        }
        let r#type = DnsType::from(value.get_u16());
        if matches!(r#type, DnsType::OPT) {
            DnsResourceRecordExtension::OPT(DnsOptResourceRecord::from(&mut *value))
        } else {
            DnsResourceRecordExtension::RR(DnsResourceRecord::from(&mut *value, name, r#type))
        }
    }
}

impl RepeatFrom<u16, &mut BytesMut> for DnsResourceRecordExtension {
    fn repeat_from(repeat : u16, value : &mut BytesMut) -> Vec<Self> {
        let mut vec = Vec::with_capacity(repeat as usize);
        for _ in 0..repeat {
            vec.push(DnsResourceRecordExtension::from(&mut *value));
        }
        vec
    }
}