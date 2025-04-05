use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;

pub struct QueryKindParseError;

impl Debug for QueryKindParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized Query Kind")
    }
}

impl Display for QueryKindParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized Query Kind")
    }
}

#[derive(Clone, Debug)]
pub enum QueryKind {
    StandardQuery = 0,
    InverseQuery = 1,
    StatusRequest = 2
}

impl TryFrom<u16> for QueryKind {
    type Error = QueryKindParseError;
    fn try_from(value: u16) -> Result<Self,QueryKindParseError> {
        match value {
            0 => Ok(QueryKind::StandardQuery),
            1 => Ok(QueryKind::InverseQuery),
            2 => Ok(QueryKind::StatusRequest),
            _ => Err(QueryKindParseError)
        }
    }
}

impl From<QueryKind> for &str {
    fn from(value: QueryKind) -> Self {
        match value {
            QueryKind::StandardQuery => "QUERY",
            QueryKind::InverseQuery => "IQUERY",
            QueryKind::StatusRequest => "STATUS"
        }
    }
}

pub struct ResponseCodeParseError;

impl Debug for ResponseCodeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized Response Code")
    }
}

impl Display for ResponseCodeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized Response Code")
    }
}

#[derive(Clone, Debug)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5
}

impl TryFrom<u16> for ResponseCode {
    type Error = ResponseCodeParseError;
    fn try_from(value: u16) -> Result<Self,ResponseCodeParseError> {
        match value {
            0 => Ok(ResponseCode::NoError),
            1 => Ok(ResponseCode::FormatError),
            2 => Ok(ResponseCode::ServerFailure),
            3 => Ok(ResponseCode::NameError),
            4 => Ok(ResponseCode::NotImplemented),
            5 => Ok(ResponseCode::Refused),
            _ => Err(ResponseCodeParseError)
        }
    }
}

#[derive(Builder, Clone, Debug)]
pub struct HeaderFlags {
    query_response: bool,
    query_kind: QueryKind, // 4 bits
    authoritative_answer: bool,
    truncated: bool,
    recursion_desired: bool,
    recursion_available: bool,
    authenticated_data: bool, // https://www.rfc-editor.org/rfc/rfc3655
    non_authenticated_data: bool, // See above
    response_code: ResponseCode, // 4 bits
}

impl HeaderFlags {
    pub fn is_query_response(&self) -> bool { self.query_response }
    pub fn get_query_kind(&self) -> QueryKind { self.query_kind.clone() }
    pub fn is_authoritative_answer(&self) -> bool { self.authoritative_answer }
    pub fn is_truncated(&self) -> bool { self.truncated }
    pub fn is_recursion_desired(&self) -> bool { self.recursion_desired }
    pub fn is_recursion_available(&self) -> bool { self.recursion_available }
    pub fn is_authenticated_data(&self) -> bool { self.authenticated_data }
    pub fn is_non_authenticated_data(&self) -> bool { self.non_authenticated_data }
    pub fn get_response_code(&self) -> ResponseCode { self.response_code.clone() }
}

impl From<u16> for HeaderFlags {
    fn from(value: u16) -> Self {
        HeaderFlags {
            query_response: value & (1 << 15) != 0,
            query_kind: QueryKind::try_from((value >> 11) & (1 << 4) - 1).expect("Expected Valid Query Kind"),
            authoritative_answer: value & (1 << 10) != 0,
            truncated: value & (1 << 9) != 0,
            recursion_desired: value & (1 << 8) != 0,
            recursion_available: value & (1 << 7) != 0,
            authenticated_data: value & (1 << 5) != 0,
            non_authenticated_data: value & (1 << 4) != 0,
            response_code: ResponseCode::try_from(value << 12 & 1).expect("Expected Valid Response Code")
        }
    }
}

impl From<HeaderFlags> for u16 {
    fn from(value: HeaderFlags) -> Self {
        let mut flags: u16 = 0;
        flags |= (value.query_response as u16) << 15;
        flags |= (value.query_kind as u16) << 11;
        flags |= (value.authoritative_answer as u16) << 10;
        flags |= (value.truncated as u16) << 9;
        flags |= (value.recursion_desired as u16) << 8;
        flags |= (value.recursion_available as u16) << 7;
        flags |= (value.authenticated_data as u16) << 5;
        flags |= (value.non_authenticated_data as u16) << 4;
        flags |= value.response_code as u16;
        flags
    }
}

#[derive(Builder, Clone, Debug)]
pub struct HeaderSection {
    id: u16,
    flags: HeaderFlags,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_record_count: u16,
}

impl HeaderSection {
    pub fn get_id(&self) -> u16 { self.id }
    pub fn get_flags(&self) -> HeaderFlags { self.flags.clone() }
    pub fn get_question_count(&self) -> u16 { self.question_count }
    pub fn get_answer_count(&self) -> u16 { self.answer_count }
    pub fn get_authority_count(&self) -> u16 { self.authority_count }
    pub fn get_additional_record_count(&self) -> u16 { self.additional_record_count }
}

impl From<&mut BytesMut> for HeaderSection {
    fn from(value: &mut BytesMut) -> Self {
        HeaderSection {
            id: value.get_u16(),
            flags: HeaderFlags::from(value.get_u16()),
            question_count: value.get_u16(),
            answer_count: value.get_u16(),
            authority_count: value.get_u16(),
            additional_record_count: value.get_u16(),
        }
    }
}

impl From<HeaderSection> for BytesMut {
    fn from(value: HeaderSection) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_u16(value.id);
        bytes.put_u16(value.flags.into());
        bytes.put_u16(value.question_count);
        bytes.put_u16(value.answer_count);
        bytes.put_u16(value.authority_count);
        bytes.put_u16(value.additional_record_count);
        bytes
    }
}