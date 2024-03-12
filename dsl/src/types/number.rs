use std::fmt::{self, Debug, Display};
use serde::{Serialize, Serializer};

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Number {
    // Int
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    // Unsigned Int
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // Float
    F32(f32),
    F64(f64),
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Self::I8(v) => serializer.serialize_i8(*v),
            Self::I16(v) => serializer.serialize_i16(*v),
            Self::I32(v) => serializer.serialize_i32(*v),
            Self::I64(v) => serializer.serialize_i64(*v),
            Self::I128(v) => serializer.serialize_i128(*v),
            Self::U8(v) => serializer.serialize_u8(*v),
            Self::U16(v) => serializer.serialize_u16(*v),
            Self::U32(v) => serializer.serialize_u32(*v),
            Self::U64(v) => serializer.serialize_u64(*v),
            Self::U128(v) => serializer.serialize_u128(*v),
            Self::F32(v) => serializer.serialize_f32(*v),
            Self::F64(v) => serializer.serialize_f64(*v),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::I8(v) => write!(f, "{}", v),
            Self::I16(v) => write!(f, "{}", v),
            Self::I32(v) => write!(f, "{}", v),
            Self::I64(v) => write!(f, "{}", v),
            Self::I128(v) => write!(f, "{}", v),
            Self::U8(v) => write!(f, "{}", v),
            Self::U16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{}", v),
            Self::U64(v) => write!(f, "{}", v),
            Self::U128(v) => write!(f, "{}", v),
            Self::F32(v) => write!(f, "{}", v),
            Self::F64(v) => write!(f, "{}", v),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::I8(v) => write!(f, "I8({})", v),
            Self::I16(v) => write!(f, "I16({})", v),
            Self::I32(v) => write!(f, "I32({})", v),
            Self::I64(v) => write!(f, "I64({})", v),
            Self::I128(v) => write!(f, "I128({})", v),
            Self::U8(v) => write!(f, "U8({})", v),
            Self::U16(v) => write!(f, "U16({})", v),
            Self::U32(v) => write!(f, "U32({})", v),
            Self::U64(v) => write!(f, "U64({})", v),
            Self::U128(v) => write!(f, "U128({})", v),
            Self::F32(v) => write!(f, "F32({})", v),
            Self::F64(v) => write!(f, "F64({})", v),
        }
    }
}
