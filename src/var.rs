use super::obj_id::ObjId;
use crate::value::{ValueData};
use crate::{ValueType, AnyData};

trait Get<T> {
    fn get(self) -> T;
}

trait Set<T> {
    fn set(&mut self, value: T);
}

pub struct Var {
    data_: ValueData,
}

impl Var {
    pub fn new() -> Var {
        Var{data_: ValueData::new()}
    }
}

impl Var {
    pub fn get_type(self) -> ValueType{
        self.data_.type_
    }

    pub fn is_integer(&self) -> bool {
        self.data_.type_ == ValueType::ValueTypeI8 || self.data_.type_ == ValueType::ValueTypeI16
            || self.data_.type_ == ValueType::ValueTypeI32 || self.data_.type_ == ValueType::ValueTypeI64
    }

    pub fn is_real(&self) -> bool{
        self.data_.type_ == ValueType::ValueTypeF32 || self.data_.type_ == ValueType::ValueTypeF64
    }

    pub fn is_number(&self) -> bool{
        self.is_integer() || self.is_real()
    }
}

impl Get<bool> for Var{
    fn get(self) -> bool {
        match self.data_.data_ {
            AnyData::VBool(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<bool> for Var {
    fn set(&mut self, value: bool) {
        self.data_.data_ = AnyData::VBool(value);
    }
}

impl Get<i8> for Var{
    fn get(self) -> i8 {
        match self.data_.data_ {
            AnyData::VI8(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<i8> for Var {
    fn set(&mut self, value: i8) {
        self.data_.data_ = AnyData::VI8(value);
    }
}

impl Get<i16> for Var{
    fn get(self) -> i16 {
        match self.data_.data_ {
            AnyData::VI16(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<i16> for Var {
    fn set(&mut self, value: i16) {
        self.data_.data_ = AnyData::VI16(value);
    }
}

impl Get<i32> for Var{
    fn get(self) -> i32 {
        match self.data_.data_ {
            AnyData::VI32(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<i32> for Var {
    fn set(&mut self, value: i32) {
        self.data_.data_ = AnyData::VI32(value);
    }
}

impl Get<i64> for Var{
    fn get(self) -> i64 {
        match self.data_.data_ {
            AnyData::VI64(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<i64> for Var {
    fn set(&mut self, value: i64) {
        self.data_.data_ = AnyData::VI64(value);
    }
}

impl Get<i128> for Var{
    fn get(self) -> i128 {
        match self.data_.data_ {
            AnyData::VI128(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<i128> for Var {
    fn set(&mut self, value: i128) {
        self.data_.data_ = AnyData::VI128(value);
    }
}

impl Get<f32> for Var{
    fn get(self) -> f32 {
        match self.data_.data_ {
            AnyData::VF32(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<f32> for Var {
    fn set(&mut self, value: f32) {
        self.data_.data_ = AnyData::VF32(value);
    }
}

impl Get<f64> for Var{
    fn get(self) -> f64 {
        match self.data_.data_ {
            AnyData::VF64(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<f64> for Var {
    fn set(&mut self, value: f64) {
        self.data_.data_ = AnyData::VF64(value);
    }
}

impl Get<String> for Var{
    fn get(self) -> String {
        match self.data_.data_ {
            AnyData::VStr(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<String> for Var {
    fn set(&mut self, value: String) {
        self.data_.data_ = AnyData::VStr(value);
    }
}

impl Get<ObjId> for Var{
    fn get(self) -> ObjId {
        match self.data_.data_ {
            AnyData::VObj(value) => value,
            _ => panic!("var get value type is error!")
        }
    }
}

impl Set<ObjId> for Var {
    fn set(&mut self, value: ObjId) {
        self.data_.data_ = AnyData::VObj(value);
    }
}


