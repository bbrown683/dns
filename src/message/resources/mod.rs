pub mod host_info;
pub mod option;
pub mod mailbox_info;
pub mod mailbox_exchange;
pub mod start_of_authority;
pub mod well_known_service;

use std::net::{Ipv4Addr, Ipv6Addr};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::message::name::Name;
use crate::message::types::Type;
use crate::message::resources::host_info::HostInfoResourceData;
use crate::message::resources::option::OptionResourceData;
use crate::message::resources::mailbox_info::MailboxInfoResourceData;
use crate::message::resources::mailbox_exchange::MailboxExchangeResourceData;
use crate::message::resources::start_of_authority::StartOfAuthorityResourceData;
use crate::message::resources::well_known_service::WellKnownServiceResourceData;

#[derive(Clone, Debug)]
pub enum ResourceData {
    CanonicalName(Name),
    HostInfo(HostInfoResourceData),
    Mailbox(Name),
    MailDestination(Name),
    MailForwarder(Name),
    MailGroup(Name),
    MailboxInfo(MailboxInfoResourceData),
    MailboxRename(Name),
    MailboxExchange(MailboxExchangeResourceData),
    Null(Bytes),
    NameServer(Name),
    Pointer(Name),
    StartOfAuthority(StartOfAuthorityResourceData),
    Text(String),
    AddressV4(Ipv4Addr),
    WellKnownService(WellKnownServiceResourceData),
    AddressV6(Ipv6Addr),
    Option(OptionResourceData)
}

impl ResourceData {
    pub fn from(bytes: &mut BytesMut, r#type: &Type) -> Self {
        match r#type {
            Type::CanonicalName => ResourceData::CanonicalName(Name::from(&mut *bytes)),
            Type::HostInfo => ResourceData::HostInfo(HostInfoResourceData::from(&mut *bytes)),
            Type::NameServer => ResourceData::NameServer(Name::from(&mut *bytes)),
            Type::AddressV4 => ResourceData::AddressV4(Ipv4Addr::from(bytes.get_u32())),
            Type::AddressV6 => ResourceData::AddressV6(Ipv6Addr::from(bytes.get_u128())),
            Type::Option => ResourceData::Option(OptionResourceData::from(bytes)),
            _ => panic!("Unknown Type for Resource Data"),
        }
    }
}

impl From<ResourceData> for BytesMut {
    fn from(value: ResourceData) -> Self {
        let mut bytes = BytesMut::new();
        match value {
            ResourceData::NameServer(domain_name ) => bytes.put(BytesMut::from(domain_name)),
            ResourceData::AddressV4(address) => bytes.put_u32(address.to_bits()),
            ResourceData::AddressV6((address)) => bytes.put_u128(address.to_bits()),
            _ => panic!("Unknown Type for Resource Data"),
        }
        bytes
    }
}