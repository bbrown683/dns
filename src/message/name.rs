use bytes::{Buf, BufMut, BytesMut};

pub enum Name {
    Sequence(String),
    Pointer(u16),
    SequencePointer(String, u16)
}

impl From<&mut BytesMut> for Name {
    fn from(value: &mut BytesMut) -> Self {
        let next = value.iter().peekable().next().expect("No more available characters");
        let pointer_bitmask: u8 = 0b11000000;
        if (next & pointer_bitmask) == pointer_bitmask {
            let pointer = value.get_u16();
            let offset_mask: u16 = 0b0011111111111111;
            let offset = pointer & offset_mask;
            Name::Pointer(offset)
        } else {
            let mut length = value.get_u8();
            let mut name = String::new();
            while length != 0 {
                let mut label_bytes = vec![0; length as usize];
                value.copy_to_slice(&mut label_bytes);

                let mut label = String::new();
                label.push_str(std::str::from_utf8(&label_bytes).unwrap());
                name.push_str(&label);
                name.push('.');

                length = value.get_u8();
            }
            Name::Sequence(name)
        }
    }
}

impl From<Name> for BytesMut {
    fn from(value: Name) -> Self {
        let mut bytes = BytesMut::new();
        match value {
            Name::Sequence(sequence) => {
                let name_pieces : Vec<&str> = sequence.split(".").collect();
                for name in name_pieces {
                    bytes.put_u8(name.len() as u8);
                    bytes.put(name.as_bytes());
                }
                bytes.put_u8(0); // Terminate label section
            },
            Name::Pointer(pointer) => {
                // TODO: Figure out how to use domain name compression with offset pointers.
            },
            Name::SequencePointer(sequence, pointer) => {

            }
        }
        bytes
    }
}