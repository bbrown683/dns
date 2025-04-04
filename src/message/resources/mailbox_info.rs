use derive_builder::Builder;

#[derive(Builder, Clone, Debug, Default)]
pub struct MailboxInfoResourceData {
    responsible_mailbox: String,
    error_mailbox: String,
}