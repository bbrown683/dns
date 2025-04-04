use bytes::{Buf, BufMut, BytesMut};
use derive_builder::Builder;

#[derive(Builder, Clone, Debug)]
pub struct DnsHeaderFlags {
    qr: bool,
    opcode: u8,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8,
    rcode: u8,
}

impl From<u16> for DnsHeaderFlags {
    fn from(value : u16) -> Self {
        DnsHeaderFlags {
            qr: (value & 0x8000) != 0,
            opcode: ((value & 0x7800) >> 11) as u8,
            aa: (value & 0x0400) != 0,
            tc: (value & 0x0200) != 0,
            rd: (value & 0x0100) != 0,
            ra: (value & 0x0080) != 0,
            z: ((value & 0x0070) >> 4) as u8,
            rcode: (value & 0x000F) as u8,
        }
    }
}

impl Into<u16> for DnsHeaderFlags {
    fn into(self) -> u16 {
        let mut value = 0;
        if self.qr {
            value |= 0x8000;
        }
        value |= (self.opcode as u16) << 11;
        if self.aa {
            value |= 0x0400;
        }
        if self.tc {
            value |= 0x0200;
        }
        if self.rd {
            value |= 0x0100;
        }
        if self.ra {
            value |= 0x0080;
        }
        value |= (self.z as u16) << 4;
        value |= self.rcode as u16;
        value
    }
}

#[derive(Builder, Clone, Debug)]
pub struct DnsHeaderSection {
    id: u16,
    flags: DnsHeaderFlags,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl From<&mut BytesMut> for DnsHeaderSection {
    fn from(value: &mut BytesMut) -> Self {
        DnsHeaderSection {
            id: value.get_u16(),
            flags: DnsHeaderFlags::from(value.get_u16()),
            qdcount: value.get_u16(),
            ancount: value.get_u16(),
            nscount: value.get_u16(),
            arcount: value.get_u16(),
        }
    }
}

impl From<DnsHeaderSection> for BytesMut {
    fn from(value: DnsHeaderSection) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put_u16(value.id);
        bytes.put_u16(value.flags.into());
        bytes.put_u16(value.qdcount);
        bytes.put_u16(value.ancount);
        bytes.put_u16(value.nscount);
        bytes.put_u16(value.arcount);
        bytes
    }
}