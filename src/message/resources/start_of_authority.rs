use derive_builder::Builder;
use std::time::Duration;
use crate::message::name::Name;

#[derive(Builder, Clone, Debug)]
pub struct StartOfAuthorityResourceData {
    primary_name_server: Name,
    responsible_authority_mailbox: Name,
    serial_number: u32,
    refresh_interval: Duration,
    retry_interval: Duration,
    expire_limit: Duration,
    minimum_time_to_live: Duration,
}