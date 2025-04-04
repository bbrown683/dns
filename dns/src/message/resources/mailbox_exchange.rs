use derive_builder::Builder;

#[derive(Builder, Clone, Debug, Default)]
pub struct MailboxExchangeResourceData {
    preference: u16,
    exchange: String,
}
