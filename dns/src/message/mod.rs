// https://en.wikipedia.org/wiki/List_of_DNS_record_types
// https://rfc-annotations.research.icann.org/
pub mod classes;
pub mod header;
pub mod question;
pub mod resource_record;
pub mod resource_data;
pub mod types;
pub mod traits;

use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;
use header::DnsHeaderSection;
use question::DnsQuestionSection;
use resource_record::DnsResourceRecordExtension;
use traits::RepeatFrom;
use std::default::Default;
use crate::message::header::{DnsHeaderFlags, DnsHeaderSectionBuilder};

// https://www.rfc-editor.org/rfc/rfc1035 DNS Specification
// https://www.rfc-editor.org/rfc/rfc8499 DNS Terminology
// https://www.rfc-editor.org/rfc/rfc9499.html DNS Terminology
#[derive(Builder, Clone, Debug)]
pub struct DnsMessage {
    pub header: DnsHeaderSection,
    pub question: Vec<DnsQuestionSection>,
    pub answer: Vec<DnsResourceRecordExtension>,
    pub authority: Vec<DnsResourceRecordExtension>,
    pub additional: Vec<DnsResourceRecordExtension>,
}

impl From<&mut BytesMut> for DnsMessage {
    fn from(value: &mut BytesMut) -> Self {
        let header = DnsHeaderSection::from(&mut *value);
        let question = DnsQuestionSection::repeat_from(header.qdcount, &mut *value);
        let answer = DnsResourceRecordExtension::repeat_from(header.ancount, &mut *value);
        let authority = DnsResourceRecordExtension::repeat_from(header.nscount, &mut *value);
        let additional = DnsResourceRecordExtension::repeat_from(header.arcount, &mut *value);
        Self {
            header,
            question,
            answer,
            authority,
            additional,
        }
    }
}

impl From<DnsMessage> for BytesMut {
    fn from(value: DnsMessage) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put(BytesMut::from(value.header));
        bytes
    }
}