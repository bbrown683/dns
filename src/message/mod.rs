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
pub mod handler;
mod name;

use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use header::HeaderSection;
use question::QuestionSection;
use records::ResourceRecordType;
use traits::RepeatToVec;
use crate::message::traits::RepeatToBytes;

// https://www.rfc-editor.org/rfc/rfc1035 DNS Specification
// https://www.rfc-editor.org/rfc/rfc8499 DNS Terminology
// https://www.rfc-editor.org/rfc/rfc9499.html DNS Terminology
// https://www.rfc-editor.org/rfc/rfc5011.html DNS Trust Anchor Updates
#[derive(Builder, Clone, Debug)]
pub struct Message {
    header: HeaderSection,
    questions: Vec<QuestionSection>,
    answers: Vec<ResourceRecordType>,
    authorities: Vec<ResourceRecordType>,
    additional_records: Vec<ResourceRecordType>,
}

impl Message {
    pub fn header(&self) -> HeaderSection {
        self.header.clone()
    }

    pub fn questions(&self) -> Vec<QuestionSection> {
        self.questions.clone()
    }

    pub fn answers(&self) -> Vec<ResourceRecordType> {
        self.answers.clone()
    }

    pub fn authorities(&self) -> Vec<ResourceRecordType> {
        self.authorities.clone()
    }

    pub fn additional_records(&self) -> Vec<ResourceRecordType> {
        self.additional_records.clone()
    }

    pub fn set_header(&mut self, header: HeaderSection) {
        self.header = header;
    }

    pub fn set_questions(&mut self, questions: Vec<QuestionSection>) {
        self.questions = questions;
    }

    pub fn set_answers(&mut self, answers: Vec<ResourceRecordType>) {
        self.answers = answers;
    }

    pub fn set_authorities(&mut self, authorities: Vec<ResourceRecordType>) {
        self.authorities = authorities;
    }

    pub fn set_additional_records(&mut self, additional_records: Vec<ResourceRecordType>) {
        self.additional_records = additional_records;
    }
}

impl From<&mut BytesMut> for Message {
    fn from(value: &mut BytesMut) -> Self {
        let header = HeaderSection::from(&mut *value);
        let question = QuestionSection::repeat_to_vec(header.question_count(), &mut *value);
        let answer = ResourceRecordType::repeat_to_vec(header.answer_count(), &mut *value);
        let authority = ResourceRecordType::repeat_to_vec(header.authority_count(), &mut *value);
        let additional = ResourceRecordType::repeat_to_vec(header.additional_record_count(), &mut *value);
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
        bytes.put(BytesMut::repeat_to_bytes(value.questions));
        bytes.put(BytesMut::repeat_to_bytes(value.answers));
        bytes.put(BytesMut::repeat_to_bytes(value.authorities));
        bytes.put(BytesMut::repeat_to_bytes(value.additional_records));
        bytes
    }
}