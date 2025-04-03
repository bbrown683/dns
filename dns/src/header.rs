use bytes::{Buf, BufMut, BytesMut};
use crate::message::DnsMessageError;

#[derive(Debug)]
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

impl TryFrom<u16> for DnsHeaderFlags {
    type Error = DnsMessageError;
    fn try_from(value : u16) -> Result<Self, DnsMessageError> {
        Ok(DnsHeaderFlags {
            qr: (value & 0x8000) != 0,
            opcode: ((value & 0x7800) >> 11) as u8,
            aa: (value & 0x0400) != 0,
            tc: (value & 0x0200) != 0,
            rd: (value & 0x0100) != 0,
            ra: (value & 0x0080) != 0,
            z: ((value & 0x0070) >> 4) as u8,
            rcode: (value & 0x000F) as u8,
        })
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

#[derive(Debug)]
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
            flags: DnsHeaderFlags::try_from(value.get_u16()).expect("invalid flags!"),
            qdcount: value.get_u16(),
            ancount: value.get_u16(),
            nscount: value.get_u16(),
            arcount: value.get_u16(),
        }
    }
}

impl Into<BytesMut> for DnsHeaderSection {
    fn into(self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);
        buf.put_u16(self.id);
        buf.put_u16(self.flags.into());
        buf.put_u16(self.qdcount);
        buf.put_u16(self.ancount);
        buf.put_u16(self.nscount);
        buf.put_u16(self.arcount);
        buf
    }
}