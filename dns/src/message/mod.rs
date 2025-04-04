// This module handles DNS message construction and deconstruction.

// https://en.wikipedia.org/wiki/List_of_DNS_record_types
// https://rfc-annotations.research.icann.org/
pub mod classes;
pub mod header;
pub mod question;
pub mod records;
pub mod resources;
pub mod types;
pub mod traits;

use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use header::HeaderSection;
use question::QuestionSection;
use records::ResourceRecordType;
use traits::RepeatFrom;
use std::default::Default;
use crate::message::header::{HeaderFlags, HeaderSectionBuilder};

// https://www.rfc-editor.org/rfc/rfc1035 DNS Specification
// https://www.rfc-editor.org/rfc/rfc8499 DNS Terminology
// https://www.rfc-editor.org/rfc/rfc9499.html DNS Terminology
#[derive(Builder, Clone, Debug)]
pub struct Message {
    pub header: HeaderSection,
    pub questions: Vec<QuestionSection>,
    pub answers: Vec<ResourceRecordType>,
    pub authorities: Vec<ResourceRecordType>,
    pub additional_records: Vec<ResourceRecordType>,
}

impl From<&mut BytesMut> for Message {
    fn from(value: &mut BytesMut) -> Self {
        let header = HeaderSection::from(&mut *value);
        let question = QuestionSection::repeat_from(header.questions, &mut *value);
        let answer = ResourceRecordType::repeat_from(header.answers, &mut *value);
        let authority = ResourceRecordType::repeat_from(header.authorities, &mut *value);
        let additional = ResourceRecordType::repeat_from(header.additional_records, &mut *value);
        Self {
            header,
            questions: question,
            answers: answer,
            authorities: authority,
            additional_records: additional,
        }
    }
}

impl From<Message> for BytesMut {
    fn from(value: Message) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put(BytesMut::from(value.header));
        bytes
    }
}