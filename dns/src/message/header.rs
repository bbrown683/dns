use bitfields::bitfield;
use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;

#[bitfield(u16, to_builder=true)]
#[derive(Clone)]
pub struct HeaderFlags {
    query_response: bool,
    #[bits(4)]
    opcode: u8,
    authoritative_answer: bool,
    truncation: bool,
    recursion_desired: bool,
    recursion_available: bool,
    #[bits(3)]
    reserved: u8,
    #[bits(4)]
    response_code: u8,
}

#[derive(Builder, Clone, Debug)]
pub struct HeaderSection {
    id: u16,
    flags: HeaderFlags,
    pub questions: u16,
    pub answers: u16,
    pub authorities: u16,
    pub additional_records: u16,
}

impl From<&mut BytesMut> for HeaderSection {
    fn from(value: &mut BytesMut) -> Self {
        HeaderSection {
            id: value.get_u16(),
            flags: HeaderFlags::from(value.get_u16()),
            questions: value.get_u16(),
            answers: value.get_u16(),
            authorities: value.get_u16(),
            additional_records: value.get_u16(),
        }
    }
}

impl From<HeaderSection> for BytesMut {
    fn from(value: HeaderSection) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_u16(value.id);
        bytes.put_u16(value.flags.into());
        bytes.put_u16(value.questions);
        bytes.put_u16(value.answers);
        bytes.put_u16(value.authorities);
        bytes.put_u16(value.additional_records);
        bytes
    }
}