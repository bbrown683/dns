#[derive(Debug, Copy, Clone)]
pub enum DnsType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    OPT = 41,
}

impl From<u16> for DnsType {
    fn from(value: u16) -> Self {
        match value {
            1 => DnsType::A,
            2 => DnsType::NS,
            3 => DnsType::MD,
            4 => DnsType::MF,
            5 => DnsType::CNAME,
            6 => DnsType::SOA,
            7 => DnsType::MB,
            8 => DnsType::MG,
            9 => DnsType::MR,
            10 => DnsType::NULL,
            11 => DnsType::WKS,
            12 => DnsType::PTR,
            13 => DnsType::HINFO,
            14 => DnsType::MINFO,
            15 => DnsType::MX,
            16 => DnsType::TXT,
            41 => DnsType::OPT,
            _ => panic!("Unknown DNS QTYPE"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum DnsQType {
    TYPE(DnsType),
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ANY = 255,
}

impl From<u16> for DnsQType {
    fn from(value: u16) -> Self {
        match value {
            1..16 | 42 => DnsQType::TYPE(DnsType::from(value)),
            252 => DnsQType::AXFR,
            253 => DnsQType::MAILB,
            254 => DnsQType::MAILA,
            255 => DnsQType::ANY,
            _ => panic!("Unknown DNS QTYPE"),
        }
    }
}