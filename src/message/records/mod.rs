pub mod option;

use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::classes::Class;
use crate::message::name::Name;
use crate::message::resources::ResourceData;
use crate::message::traits::{RepeatToBytes, RepeatToVec};
use crate::message::types::Type;
use crate::message::records::option::OptionResourceRecord;

#[derive(Builder, Clone, Debug)]
pub struct ResourceRecord {
    name: Name,
    r#type: Type,
    class: Class,
    time_to_live: i32,
    resource_data: ResourceData
}

impl ResourceRecord {
    fn from(value: &mut BytesMut, name : Name, r#type : Type) -> Self {
        let class = Class::from(value.get_u16());
        let time_to_live = value.get_i32();
        value.get_u16();
        let resource_data = ResourceData::from(value, &r#type);
        ResourceRecord {
            name,
            r#type,
            class,
            time_to_live,
            resource_data,
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn r#type(&self) -> Type {
        self.r#type
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn time_to_live(&self) -> i32 {
        self.time_to_live
    }

    pub fn resource_data(&self) -> &ResourceData {
        &self.resource_data
    }

    pub fn set_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn set_type(&mut self, r#type : Type) {
        self.r#type = r#type;
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class;
    }

    pub fn set_time_to_live(&mut self, time_to_live: i32) {
        self.time_to_live = time_to_live;
    }

    pub fn set_resource_data(&mut self, resource_data: ResourceData) {
        self.resource_data = resource_data;
    }
}

impl From<ResourceRecord> for BytesMut {
    fn from(value: ResourceRecord) -> Self {
        let mut bytes = BytesMut::new();

        bytes.put(BytesMut::from(value.name));
        bytes.put_u16(value.r#type as u16);
        bytes.put_u16(value.class as u16);
        bytes.put_i32(value.time_to_live);

        let resource_data = BytesMut::from(value.resource_data);
        bytes.put_u16(resource_data.len() as u16);
        bytes.put(resource_data);
        bytes
    }
}

#[derive(Clone, Debug)]
pub enum ResourceRecordType {
    ResourceRecord(ResourceRecord),
    OptionResourceRecord(OptionResourceRecord),
}

impl From<&mut BytesMut> for ResourceRecordType {
    fn from(value: &mut BytesMut) -> Self {
        let name = Name::from(&mut *value);
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
            ResourceRecordType::ResourceRecord(resource_record) => BytesMut::from(resource_record),
            ResourceRecordType::OptionResourceRecord(option_resource_record) => BytesMut::from(option_resource_record),
        }
    }
}

impl RepeatToVec<u16, &mut BytesMut> for ResourceRecordType {
    fn repeat_to_vec(repeat : u16, value : &mut BytesMut) -> Vec<Self> {
        let mut vec = Vec::with_capacity(repeat as usize);
        for _ in 0..repeat {
            vec.push(ResourceRecordType::from(&mut *value));
        }
        vec
    }
}

impl RepeatToBytes<ResourceRecordType> for BytesMut {
    fn repeat_to_bytes(value: Vec<ResourceRecordType>) -> BytesMut {
        let mut bytes = BytesMut::new();
        for resource_record in value {
            bytes.put(BytesMut::from(resource_record));
        }
        bytes
    }
}