use std::fmt;

pub enum LoxValue {
    LoxNumber(f64),
    LoxString(String),
    LoxBool(bool),
    LoxNil
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::LoxNil => write!(f, "nil"),
            LoxValue::LoxBool(b) => write!(f, "{}", b),
            LoxValue::LoxNumber(n) => write!(f, "{}", n),
            LoxValue::LoxString(s) => write!(f, "{}", s)
        }
    }
}
