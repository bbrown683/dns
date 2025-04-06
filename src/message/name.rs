use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub enum Name {
    Sequence(String),
    Pointer(u16),
    SequencePointer(String, u16)
}

impl Name {
    fn is_pointer_next(bytes : &mut BytesMut) -> bool {
        let next = bytes.iter().peekable().next().expect("No more available characters");
        let pointer_bitmask: u8 = 0b11000000;
        next & pointer_bitmask == pointer_bitmask
    }
}

impl From<&mut BytesMut> for Name {
    fn from(value: &mut BytesMut) -> Self {
        if Self::is_pointer_next(value) {
            Name::Pointer(value.get_u16())
        } else {
            let mut length = value.get_u8();
            let mut name = String::new();
            while length != 0 {
                let mut label_bytes = vec![0; length as usize];
                value.copy_to_slice(&mut label_bytes);

                let mut label = String::new();
                label.push_str(std::str::from_utf8(&label_bytes).unwrap());
                name.push_str(&label);

                // Check if a pointer is next.
                if Self::is_pointer_next(value) {
                    return Name::SequencePointer(name, value.get_u16())
                }
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
            },
            Name::Pointer(pointer) => {
                bytes.put_u16(pointer);
            },
            Name::SequencePointer(sequence, pointer) => {
                let name_pieces : Vec<&str> = sequence.split(".").collect();
                for name in name_pieces {
                    bytes.put_u8(name.len() as u8);
                    bytes.put(name.as_bytes());
                }
                bytes.put_u16(pointer);
            }
        }
        bytes
    }
}