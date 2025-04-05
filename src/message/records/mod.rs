pub mod option;

use std::time::Duration;
use bytes::{Buf, BytesMut};
use derive_builder::Builder;
use crate::message::classes::Class;
use crate::message::resources::ResourceData;
use crate::message::traits::RepeatFrom;
use crate::message::types::Type;
use crate::message::records::option::OptionResourceRecord;

#[derive(Builder, Clone, Debug)]
pub struct ResourceRecord {
    name: String,
    r#type: Type,
    class: Class,
    time_to_live: Duration,
    resource_data_bytes: u16,
    resource_data: ResourceData
}

impl ResourceRecord {
    fn from(value: &mut BytesMut, name : String, r#type : Type) -> Self {
        let class = Class::from(value.get_u16());
        let time_to_live = Duration::from_secs(value.get_i32() as u64);
        let resource_data_bytes = value.get_u16();
        let resource_data = ResourceData::from(value, &r#type);
        ResourceRecord {
            name,
            r#type,
            class,
            time_to_live,
            resource_data_bytes,
            resource_data,
        }
    }
}

impl From<ResourceRecord> for BytesMut {
    fn from(value: ResourceRecord) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub enum ResourceRecordType {
    ResourceRecord(ResourceRecord),
    OptionResourceRecord(OptionResourceRecord),
}

impl From<&mut BytesMut> for ResourceRecordType {
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
        let r#type = Type::from(value.get_u16());
        if matches!(r#type, Type::Option) {
            ResourceRecordType::OptionResourceRecord(OptionResourceRecord::from(&mut *value))
        } else {
            ResourceRecordType::ResourceRecord(ResourceRecord::from(&mut *value, name, r#type))
        }
    }
}

impl From<ResourceRecordType> for BytesMut {
    fn from(value: ResourceRecordType) -> Self {
        match value {
            // ResourceRecordType::ResourceRecord(resource_record) => ResourceRecord::from(resource_record),
            ResourceRecordType::OptionResourceRecord(option_resource_record) => OptionResourceRecord::from(option_resource_record).into(),
            _ => todo!()
        }
    }
}

impl RepeatFrom<u16, &mut BytesMut> for ResourceRecordType {
    fn repeat_from(repeat : u16, value : &mut BytesMut) -> Vec<Self> {
        let mut vec = Vec::with_capacity(repeat as usize);
        for _ in 0..repeat {
            vec.push(ResourceRecordType::from(&mut *value));
        }
        vec
    }
}