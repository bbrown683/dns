#[derive(Debug, Clone)]
pub enum Class {
    Internet = 1,
    CSNET = 2,
    CHAOS = 3,
    Hesiod = 4,
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Class::Internet,
            2 => Class::CSNET,
            3 => Class::CHAOS,
            4 => Class::Hesiod,
            _ => panic!("Unknown DNS Class {:?}", value),
        }
    }
}

impl From<Class> for &str {
    fn from(value: Class) -> Self {
        match value {
            Class::Internet => "IN",
            Class::CSNET => "CS",
            Class::CHAOS => "CH",
            Class::Hesiod => "HS",
        }
    }
}

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum QueryClass {
    Any = 255,
    Class(Class),
}

impl From<u16> for QueryClass {
    fn from(value: u16) -> Self {
        match value {
            ..4 => QueryClass::Class(Class::from(value)),
            255 => QueryClass::Any,
            _ => panic!("Unknown DNS Query Class {:?}", value),
        }
    }
}

impl From<QueryClass> for u16 {
    fn from(value: QueryClass) -> Self {
        match value {
            QueryClass::Any => 255,
            QueryClass::Class(class) => class as u16,
        }
    }
}

impl From<QueryClass> for &str {
    fn from(value: QueryClass) -> Self {
        match value {
            QueryClass::Any => "*",
            QueryClass::Class(class) => class.into(),
        }
    }
}