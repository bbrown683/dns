// https://en.wikipedia.org/wiki/List_of_DNS_record_types
// https://rfc-annotations.research.icann.org/

use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::header::DnsHeaderSection;
use crate::question::DnsQuestionSection;
use crate::resource_record::DnsResourceRecordExtension;
use crate::traits::RepeatFrom;

#[derive(Debug)]
pub struct DnsMessageError {
    message: String,
}

impl DnsMessageError {
    fn new(message: &str) -> Self {
        DnsMessageError {
            message: message.to_string(),
        }
    }
}

// https://www.rfc-editor.org/rfc/rfc1035 DNS Specification
// https://www.rfc-editor.org/rfc/rfc8499 DNS Terminology
// https://www.rfc-editor.org/rfc/rfc9499.html DNS Terminology
#[derive(Debug)]
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

impl Into<BytesMut> for DnsMessage {
    fn into(self) -> BytesMut {
        let mut buf = BytesMut::zeroed(512);
        buf
    }
}