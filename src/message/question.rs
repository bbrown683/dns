use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::classes::QueryClass;
use crate::message::traits::RepeatFrom;
use crate::message::types::QueryType;

#[derive(Builder, Clone, Debug)]
pub struct QuestionSection {
    name: Vec<String>,
    r#type: QueryType,
    class: QueryClass,
}

impl From<&mut BytesMut> for QuestionSection {
    fn from(value: &mut BytesMut) -> Self {
        let mut name = Vec::new();

        let mut length = value.get_u8();
        while length != 0 {
            let mut label_bytes = vec![0; length as usize];
            value.copy_to_slice(&mut label_bytes);

            let mut label = String::new();
            label.push_str(std::str::from_utf8(&label_bytes).unwrap());
            name.push(label);

            length = value.get_u8();
        }

        QuestionSection {
            name,
            r#type: QueryType::from(value.get_u16()),
            class: QueryClass::from(value.get_u16()),
        }
    }
}

impl RepeatFrom<u16, &mut BytesMut> for QuestionSection {
    fn repeat_from(repeat : u16, value : &mut BytesMut) -> Vec<Self> {
        let mut vec = Vec::with_capacity(repeat as usize);
        for _ in 0..repeat {
            vec.push(QuestionSection::from(&mut *value));
        }
        vec
    }
}

impl From<QuestionSection> for BytesMut {
    fn from(value: QuestionSection) -> Self {
        let mut bytes = BytesMut::new();
        for label in value.name {
            bytes.put_u8(label.len() as u8);
            bytes.put(label.as_bytes());
        }
        bytes.put_u8(0);
        bytes.put_u16(value.r#type.into());
        bytes.put_u16(value.class.into());
        bytes
    }
}