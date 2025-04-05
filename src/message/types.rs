#[derive(Debug, Copy, Clone)]
pub enum Type {
    AddressV4 = 1,
    NameServer = 2,
    MailDestination = 3,
    MailForwarder = 4,
    CanonicalName = 5,
    StartOfAuthority = 6,
    Mailbox = 7,
    MailGroup = 8,
    MailRename = 9,
    Null = 10,
    WellKnownService = 11,
    Pointer = 12,
    HostInfo = 13,
    MailboxInfo = 14,
    MailboxExchange = 15,
    Text = 16,
    AddressV6 = 28,
    Option = 41,
    HTTPS = 65,
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Type::AddressV4,
            2 => Type::NameServer,
            3 => Type::MailDestination,
            4 => Type::MailForwarder,
            5 => Type::CanonicalName,
            6 => Type::StartOfAuthority,
            7 => Type::Mailbox,
            8 => Type::MailGroup,
            9 => Type::MailRename,
            10 => Type::Null,
            11 => Type::WellKnownService,
            12 => Type::Pointer,
            13 => Type::HostInfo,
            14 => Type::MailboxInfo,
            15 => Type::MailboxExchange,
            16 => Type::Text,
            28 => Type::AddressV6,
            41 => Type::Option,
            65 => Type::HTTPS,
            _ => panic!("Unknown DNS Type {:?}", value),
        }
    }
}

impl From<Type> for &str {
    fn from(value: Type) -> Self {
        match value {
            Type::AddressV4 => "A",
            Type::NameServer => "NS",
            Type::MailDestination => "MD",
            Type::MailForwarder => "MF",
            Type::CanonicalName => "CNAME",
            Type::StartOfAuthority => "SOA",
            Type::Mailbox => "MB",
            Type::MailGroup => "MG",
            Type::MailRename => "MR",
            Type::Null => "NULL",
            Type::WellKnownService => "WKS",
            Type::Pointer => "PTR",
            Type::HostInfo => "HINFO",
            Type::MailboxInfo => "MINFO",
            Type::MailboxExchange => "MX",
            Type::Text => "TXT",
            Type::AddressV6 => "AAAA",
            Type::Option => "OPT",
            Type::HTTPS => "HTTPS",
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum QueryType {
    Type(Type),
    TransferOfZone = 252,
    Mailbox = 253,
    MailAgent = 254,
    Any = 255,
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            ..252 => QueryType::Type(Type::from(value)),
            252 => QueryType::TransferOfZone,
            253 => QueryType::Mailbox,
            254 => QueryType::MailAgent,
            255 => QueryType::Any,
            _ => panic!("Unknown DNS Query Type {:?}", value),
        }
    }
}

impl From<QueryType> for u16 {
    fn from(value: QueryType) -> Self {
        match value {
            QueryType::Type(t) => t as u16,
            QueryType::TransferOfZone => 252,
            QueryType::Mailbox => 253,
            QueryType::MailAgent => 254,
            QueryType::Any => 255,
        }
    }
}

impl From<QueryType> for &str {
    fn from(value: QueryType) -> Self {
        match value {
            QueryType::Type(t) => t.into(),
            QueryType::TransferOfZone => "AXFR",
            QueryType::Mailbox => "MAILB",
            QueryType::MailAgent => "MAILA",
            QueryType::Any => "*",
        }
    }
}