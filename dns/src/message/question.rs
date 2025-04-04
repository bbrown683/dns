use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::classes::DnsQClass;
use crate::message::traits::RepeatFrom;
use crate::message::types::DnsQType;

#[derive(Builder, Clone, Debug)]
pub struct DnsQuestionSection {
    qname: Vec<String>,
    qtype: DnsQType,
    qclass: DnsQClass,
}

impl From<&mut BytesMut> for DnsQuestionSection {
    fn from(value: &mut BytesMut) -> Self {
        let mut qname = Vec::new();

        let mut length = value.get_u8();
        while length != 0 {
            let mut label_bytes = vec![0; length as usize];
            value.copy_to_slice(&mut label_bytes);

            let mut label = String::new();
            label.push_str(std::str::from_utf8(&label_bytes).unwrap());
            qname.push(label);

            length = value.get_u8();
        }

        DnsQuestionSection {
            qname,
            qtype: DnsQType::from(value.get_u16()),
            qclass: DnsQClass::from(value.get_u16()),
        }
    }
}

impl RepeatFrom<u16, &mut BytesMut> for DnsQuestionSection {
    fn repeat_from(repeat : u16, value : &mut BytesMut) -> Vec<Self> {
        let mut vec = Vec::with_capacity(repeat as usize);
        for _ in 0..repeat {
            vec.push(DnsQuestionSection::from(&mut *value));
        }
        vec
    }
}

impl From<DnsQuestionSection> for BytesMut {
    fn from(value: DnsQuestionSection) -> Self {
        let mut bytes = BytesMut::new();
        for label in value.qname {
            bytes.put_u8(label.len() as u8);
            bytes.put(label.as_bytes());
        }
        bytes.put_u8(0);
        bytes.put_u16(value.qtype.into());
        bytes.put_u16(value.qclass.into());
        bytes
    }
}