use crate::message::{Message, MessageBuilder};
use crate::message::header::{HeaderFlagsBuilder, HeaderSectionBuilder, QueryKind, ResponseCode};
use crate::message::records::option::OptionResourceRecordCode;
use crate::message::records::ResourceRecordType;
use crate::message::resources::option::{OptionRecordDataOption, OptionResourceData};

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
            .recursion_desired(header.get_flags().is_recursion_desired())
            .recursion_available(true)
            .authenticated_data(true)
            .non_authenticated_data(false)
            .response_code(ResponseCode::NoError)
            .build()
            .unwrap();
        let header = HeaderSectionBuilder::default()
            .id(header.get_id())
            .flags(flags)
            .question_count(header.get_question_count())
            .answer_count(0)
            .authority_count(0)
            .additional_record_count(header.get_additional_record_count())
            .build()
            .unwrap();

        let questions = &request.questions;
        let answers = &request.answers;
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