#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum DnsClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl From<u16> for DnsClass {
    fn from(value: u16) -> Self {
        match value {
            1 => DnsClass::IN,
            2 => DnsClass::CS,
            3 => DnsClass::CH,
            4 => DnsClass::HS,
            _ => panic!("Unknown DNS CLASS {:?}", value),
        }
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum DnsQClass {
    ANY = 255,
    CLASS(DnsClass),
}

impl From<u16> for DnsQClass {
    fn from(value: u16) -> Self {
        match value {
            1..4 => DnsQClass::CLASS(DnsClass::from(value)),
            255 => DnsQClass::ANY,
            _ => panic!("Unknown DNS QCLASS"),
        }
    }
}