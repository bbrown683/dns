use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use crate::message::{Message, MessageBuilder};
use crate::message::classes::Class;
use crate::message::header::{HeaderFlagsBuilder, HeaderSectionBuilder, QueryKind, ResponseCode};
use crate::message::records::option::OptionResourceRecordCode;
use crate::message::records::{ResourceRecordBuilder, ResourceRecordType};
use crate::message::resources::option::{OptionRecordDataOption, OptionResourceData};
use crate::message::resources::ResourceData;
use crate::message::types::Type;

// TODO: Query root domains
pub struct Handler;

impl Handler {
    pub fn get_response(request : &Message) -> Message {
        let header = &request.header;
        let flags = HeaderFlagsBuilder::default()
            .query_response(true)
            .query_kind(QueryKind::StandardQuery)
            .truncated(false)
            .authoritative_answer(false)
            .recursion_desired(header.flags().recursion_desired())
            .recursion_available(true)
            .authenticated_data(true)
            .non_authenticated_data(false)
            .response_code(ResponseCode::NoError)
            .build()
            .unwrap();
        let header = HeaderSectionBuilder::default()
            .id(header.id())
            .flags(flags)
            .question_count(header.question_count())
            .answer_count(1)
            .authority_count(0)
            .additional_record_count(header.additional_record_count())
            .build()
            .unwrap();

        let questions = &request.questions;
        let answers = vec![ResourceRecordType::ResourceRecord(
            ResourceRecordBuilder::default()
                .name(String::from("amazon.com"))
                .r#type(Type::AddressV4)
                .class(Class::Internet)
                .time_to_live(300)
                .resource_data_bytes(4)
                .resource_data(ResourceData::AddressV4(Ipv4Addr::from_str("142.251.15.113").expect("Failed to parse IP Address")))
                .build().unwrap()
        )];
        let authorities = &request.authorities;
        let additional_records = &request.additional_records;
        MessageBuilder::default()
            .header(header)
            .questions(questions.clone())
            .answers(answers.clone())
            .authorities(authorities.clone())
            .additional_records(additional_records.clone())
            .build()
            .unwrap()
    }
}