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
}

impl From<&mut BytesMut> for Message {
    fn from(value: &mut BytesMut) -> Self {
        let header = HeaderSection::from(&mut *value);
        let question = QuestionSection::repeat_from(header.get_question_count(), &mut *value);
        let answer = ResourceRecordType::repeat_from(header.get_answer_count(), &mut *value);
        let authority = ResourceRecordType::repeat_from(header.get_authority_count(), &mut *value);
        let additional = ResourceRecordType::repeat_from(header.get_additional_record_count(), &mut *value);
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

        let questions = value.questions;
        for question in questions {
            bytes.put(BytesMut::from(question));
        }

        let answers = value.answers;
        for answer in answers {
            bytes.put(BytesMut::from(answer));
        }

        let authorities = value.authorities;
        for authority in authorities {
            bytes.put(BytesMut::from(authority))
        }

        let additional_records = value.additional_records;
        for additional_record in additional_records {
            bytes.put(BytesMut::from(additional_record));
        }
        bytes
    }
}