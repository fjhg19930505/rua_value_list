use crate::ObjId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueType{
    ValueTypeUnKnow,		    // 未知
    ValueTypeBool,			    // 布尔
    ValueTypeU8,                // 8位无符号
    ValueTypeU16,               // 16位无符号
    ValueTypeU32,               // 32位无符号
    ValueTypeU64,               // 64位无符号
    ValueTypeU128,              // 128位无符号
    ValueTypeI8,			    // 1字节
    ValueTypeI16,			    // 2字节
    ValueTypeI32,			    // 32位整数
    ValueTypeI64,			    // 64位整数
    ValueTypeI128,			    // 128位整数
    ValueTypeF32,			    // 单精度浮点数
    ValueTypeF64,			    // 双精度浮点数
    ValueTypeStr,			    // 字符串
    ValueTypeObj,			    // 对象号
    ValueTypeMax,
}

#[derive(Debug, Clone)]
pub enum AnyData {
    VUnknown,
    VU8(u8),
    VU16(u16),
    VU32(u32),
    VU64(u64),
    VU128(u128),
    VI8(i8),
    VI16(i16),
    VI32(i32),
    VI64(i64),
    VI128(i128),
    VF32(f32),
    VF64(f64),
    VBool(bool),
    VObj(ObjId),
    VStr(String),
}

impl From<bool> for AnyData {
    fn from(value: bool) -> Self {
        AnyData::VBool(value)
    }
}

impl From<u8> for AnyData {
    fn from(value: u8) -> Self {
        AnyData::VU8(value)
    }
}

impl From<u16> for AnyData {
    fn from(value: u16) -> Self {
        AnyData::VU16(value)
    }
}

impl From<u32> for AnyData {
    fn from(value: u32) -> Self {
        AnyData::VU32(value)
    }
}

impl From<u64> for AnyData {
    fn from(value: u64) -> Self {
        AnyData::VU64(value)
    }
}

impl From<u128> for AnyData {
    fn from(value: u128) -> Self {
        AnyData::VU128(value)
    }
}

impl From<i8> for AnyData {
    fn from(value: i8) -> Self {
        AnyData::VI8(value)
    }
}

impl From<i16> for AnyData {
    fn from(value: i16) -> Self {
        AnyData::VI16(value)
    }
}

impl From<i32> for AnyData {
    fn from(value: i32) -> Self {
        AnyData::VI32(value)
    }
}

impl From<i64> for AnyData {
    fn from(value: i64) -> Self {
        AnyData::VI64(value)
    }
}

impl From<i128> for AnyData {
    fn from(value: i128) -> Self {
        AnyData::VI128(value)
    }
}

impl From<f32> for AnyData {
    fn from(value: f32) -> Self {
        AnyData::VF32(value)
    }
}

impl From<f64> for AnyData {
    fn from(value: f64) -> Self {
        AnyData::VF64(value)
    }
}

impl From<String> for AnyData {
    fn from(value: String) -> Self {
        AnyData::VStr(value)
    }
}

impl From<ObjId> for AnyData {
    fn from(value: ObjId) -> Self {
        AnyData::VObj(value)
    }
}

#[derive(Debug, Clone)]
pub struct ValueData {
    pub type_: ValueType,
    pub data_: AnyData,
}

impl ValueData {
    pub(crate) fn new() -> Self {
        ValueData{
            type_: ValueType::ValueTypeUnKnow,
            data_: AnyData::VUnknown,
        }
    }
}