use crate::value::{AnyData, ValueData, ValueType};
use crate::ObjId;
use std::ops::Add;

pub trait Get<T> {
    fn get(&self, index: usize) -> T;
}

pub trait Set<T> {
    fn set(&mut self, index: usize, value: T);
}

pub trait Put<T> {
    fn put(&mut self, value: T) -> &mut Self;
}

pub trait Init<T> {
    fn init(t: T) -> VarList;
}

#[derive(Debug, Clone)]
pub struct VarList {
    data_stack_: Vec<ValueData>,
}

impl VarList {
    pub fn new() -> VarList {
        VarList {
            data_stack_: vec![],
        }
    }
}

impl VarList {
    // 合并列表
    pub fn combine(self, src: Self) -> VarList {
        let count = src.get_count();
        let temp = self.inner_append(src, 0, count).clone();
        temp
    }

    fn inner_append(self, src: VarList, start: usize, end: usize) -> VarList {
        let mut temp = self.clone();
        for i in start..end {
            match src.get_type(i) {
                ValueType::ValueTypeBool => {
                    let value: bool = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeU8 => {
                    let value: u8 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeU16 => {
                    let value: u16 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeU32 => {
                    let value: u32 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeU64 => {
                    let value: u64 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeU128 => {
                    let value: u128 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeI8 => {
                    let value: i8 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeI16 => {
                    let value: i16 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeI32 => {
                    let value: i32 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeI64 => {
                    let value: i64 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeI128 => {
                    let value: i128 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeF32 => {
                    let value: f32 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeF64 => {
                    let value: f64 = src.get(i);
                    temp = temp.add(value);
                }
                ValueType::ValueTypeStr => {
                    let value: String = src.get(i);
                    temp = temp.add(value.clone());
                }
                ValueType::ValueTypeObj => {
                    let value: ObjId = src.get(i);
                    temp = temp.add(value);
                }
                _ => {
                    println!("type is not valid");
                }
            }
        }

        temp
    }

    // 清空
    pub fn clear(&mut self) {
        self.data_stack_.clear();
    }

    // 是否为空
    pub fn is_empty(&self) -> bool {
        self.get_count() == 0
    }

    // 数据长度
    pub fn get_count(&self) -> usize {
        self.data_stack_.len()
    }

    // 数据类型
    pub fn get_type(&self, index: usize) -> ValueType {
        if index >= self.data_stack_.len() {
            return ValueType::ValueTypeUnKnow;
        }

        self.data_stack_[index].type_.clone()
    }

    // 获取内存用量
    fn get_memory_usage<'a>(&self) -> usize {
        let mut mem: usize = 0;
        for i in 0..self.get_count() {
            match self.get_type(i) {
                ValueType::ValueTypeBool => mem += 2,
                ValueType::ValueTypeU8 => mem += 4,
                ValueType::ValueTypeU16 => mem += 8,
                ValueType::ValueTypeU32 => mem += 16,
                ValueType::ValueTypeU64 => mem += 32,
                ValueType::ValueTypeU128 => mem += 64,
                ValueType::ValueTypeI8 => mem += 4,
                ValueType::ValueTypeI16 => mem += 8,
                ValueType::ValueTypeI32 => mem += 16,
                ValueType::ValueTypeI64 => mem += 32,
                ValueType::ValueTypeI128 => mem += 64,
                ValueType::ValueTypeF32 => mem += 16,
                ValueType::ValueTypeF64 => mem += 32,
                ValueType::ValueTypeStr => {
                    let value: String = self.get(i);
                    mem += value.len() * 2;
                }
                ValueType::ValueTypeObj => mem += 32,
                _ => println!("type not invalid"),
            }
        }
        mem
    }
}

impl Init<Vec<bool>> for VarList {
    fn init(vec: Vec<bool>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<bool> for VarList {
    type Output = VarList;

    fn add(self, value: bool) -> Self::Output {
        let mut result: Self::Output = self;
        let var = ValueData {
            type_: ValueType::ValueTypeBool,
            data_: AnyData::VBool(value),
        };
        result.data_stack_.push(var);
        result
    }
}

impl Get<bool> for VarList {
    fn get(&self, index: usize) -> bool {
        if index >= self.data_stack_.len() {
            return false;
        }
        match self.data_stack_[index].data_ {
            AnyData::VBool(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<bool> for VarList {
    fn set(&mut self, index: usize, value: bool) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeBool;
        self.data_stack_[index].data_ = AnyData::VBool(value);
    }
}

impl Put<bool> for VarList {
    fn put(&mut self, value: bool) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeBool,
            data_: AnyData::VBool(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<u8>> for VarList {
    fn init(vec: Vec<u8>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<u8> for VarList {
    type Output = VarList;

    fn add(self, value: u8) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeU8,
            data_: AnyData::VU8(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<u8> for VarList {
    fn get(&self, index: usize) -> u8 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VU8(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<u8> for VarList {
    fn set(&mut self, index: usize, value: u8) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeU8;
        self.data_stack_[index].data_ = AnyData::VU8(value);
    }
}

impl Put<u8> for VarList {
    fn put(&mut self, value: u8) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeU8,
            data_: AnyData::VU8(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<u16>> for VarList {
    fn init(vec: Vec<u16>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<u16> for VarList {
    type Output = VarList;

    fn add(self, value: u16) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeU16,
            data_: AnyData::VU16(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<u16> for VarList {
    fn get(&self, index: usize) -> u16 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VU16(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<u16> for VarList {
    fn set(&mut self, index: usize, value: u16) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeU16;
        self.data_stack_[index].data_ = AnyData::VU16(value);
    }
}

impl Put<u16> for VarList {
    fn put(&mut self, value: u16) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeU16,
            data_: AnyData::VU16(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<u32>> for VarList {
    fn init(vec: Vec<u32>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<u32> for VarList {
    type Output = VarList;

    fn add(self, value: u32) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeU32,
            data_: AnyData::VU32(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<u32> for VarList {
    fn get(&self, index: usize) -> u32 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VU32(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<u32> for VarList {
    fn set(&mut self, index: usize, value: u32) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeU32;
        self.data_stack_[index].data_ = AnyData::VU32(value);
    }
}

impl Put<u32> for VarList {
    fn put(&mut self, value: u32) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeU32,
            data_: AnyData::VU32(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<u64>> for VarList {
    fn init(vec: Vec<u64>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<u64> for VarList {
    type Output = VarList;

    fn add(self, value: u64) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeU64,
            data_: AnyData::VU64(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<u64> for VarList {
    fn get(&self, index: usize) -> u64 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VU64(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<u64> for VarList {
    fn set(&mut self, index: usize, value: u64) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeU64;
        self.data_stack_[index].data_ = AnyData::VU64(value);
    }
}

impl Init<Vec<u128>> for VarList {
    fn init(vec: Vec<u128>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Put<u64> for VarList {
    fn put(&mut self, value: u64) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeU64,
            data_: AnyData::VU64(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Add<u128> for VarList {
    type Output = VarList;

    fn add(self, value: u128) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeU128,
            data_: AnyData::VU128(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<u128> for VarList {
    fn get(&self, index: usize) -> u128 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VU128(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<u128> for VarList {
    fn set(&mut self, index: usize, value: u128) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeU128;
        self.data_stack_[index].data_ = AnyData::VU128(value);
    }
}

impl Put<u128> for VarList {
    fn put(&mut self, value: u128) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeU128,
            data_: AnyData::VU128(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<i8>> for VarList {
    fn init(vec: Vec<i8>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<i8> for VarList {
    type Output = VarList;

    fn add(self, value: i8) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeI8,
            data_: AnyData::VI8(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<i8> for VarList {
    fn get(&self, index: usize) -> i8 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VI8(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<i8> for VarList {
    fn set(&mut self, index: usize, value: i8) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeI8;
        self.data_stack_[index].data_ = AnyData::VI8(value);
    }
}

impl Put<i8> for VarList {
    fn put(&mut self, value: i8) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeI8,
            data_: AnyData::VI8(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<i16>> for VarList {
    fn init(vec: Vec<i16>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<i16> for VarList {
    type Output = VarList;

    fn add(self, value: i16) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeI16,
            data_: AnyData::VI16(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<i16> for VarList {
    fn get(&self, index: usize) -> i16 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VI16(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<i16> for VarList {
    fn set(&mut self, index: usize, value: i16) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeI16;
        self.data_stack_[index].data_ = AnyData::VI16(value);
    }
}

impl Put<i16> for VarList {
    fn put(&mut self, value: i16) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeI16,
            data_: AnyData::VI16(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<i32>> for VarList {
    fn init(vec: Vec<i32>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<i32> for VarList {
    type Output = VarList;

    fn add(self, value: i32) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeI32,
            data_: AnyData::VI32(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<i32> for VarList {
    fn get(&self, index: usize) -> i32 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VI32(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<i32> for VarList {
    fn set(&mut self, index: usize, value: i32) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeI32;
        self.data_stack_[index].data_ = AnyData::VI32(value);
    }
}

impl Put<i32> for VarList {
    fn put(&mut self, value: i32) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeI32,
            data_: AnyData::VI32(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<i64>> for VarList {
    fn init(vec: Vec<i64>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<i64> for VarList {
    type Output = VarList;

    fn add(self, value: i64) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeI64,
            data_: AnyData::VI64(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<i64> for VarList {
    fn get(&self, index: usize) -> i64 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VI64(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<i64> for VarList {
    fn set(&mut self, index: usize, value: i64) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeI64;
        self.data_stack_[index].data_ = AnyData::VI64(value);
    }
}

impl Put<i64> for VarList {
    fn put(&mut self, value: i64) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeI64,
            data_: AnyData::VI64(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<i128>> for VarList {
    fn init(vec: Vec<i128>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<i128> for VarList {
    type Output = VarList;

    fn add(self, value: i128) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeI128,
            data_: AnyData::VI128(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<i128> for VarList {
    fn get(&self, index: usize) -> i128 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VI128(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<i128> for VarList {
    fn set(&mut self, index: usize, value: i128) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeI128;
        self.data_stack_[index].data_ = AnyData::VI128(value);
    }
}

impl Put<i128> for VarList {
    fn put(&mut self, value: i128) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeI128,
            data_: AnyData::VI128(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<f32>> for VarList {
    fn init(vec: Vec<f32>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<f32> for VarList {
    type Output = VarList;

    fn add(self, value: f32) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeF32,
            data_: AnyData::VF32(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<f32> for VarList {
    fn get(&self, index: usize) -> f32 {
        if index >= self.data_stack_.len() {
            return 0.0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VF32(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<f32> for VarList {
    fn set(&mut self, index: usize, value: f32) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeF32;
        self.data_stack_[index].data_ = AnyData::VF32(value);
    }
}

impl Put<f32> for VarList {
    fn put(&mut self, value: f32) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeF32,
            data_: AnyData::VF32(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<f64>> for VarList {
    fn init(vec: Vec<f64>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<f64> for VarList {
    type Output = VarList;

    fn add(self, value: f64) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeF64,
            data_: AnyData::VF64(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<f64> for VarList {
    fn get(&self, index: usize) -> f64 {
        if index >= self.data_stack_.len() {
            return 0.0;
        }
        match self.data_stack_[index].data_ {
            AnyData::VF64(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<f64> for VarList {
    fn set(&mut self, index: usize, value: f64) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeF64;
        self.data_stack_[index].data_ = AnyData::VF64(value);
    }
}

impl Put<f64> for VarList {
    fn put(&mut self, value: f64) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeF64,
            data_: AnyData::VF64(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<String>> for VarList {
    fn init(vec: Vec<String>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<String> for VarList {
    type Output = VarList;

    fn add(self, value: String) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeStr,
            data_: AnyData::VStr(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<String> for VarList {
    fn get(&self, index: usize) -> String {
        if index >= self.data_stack_.len() {
            return String::new();
        }

        let data = &self.data_stack_[index];
        let value = data.data_.clone();
        match value {
            AnyData::VStr(value) => value.clone(),
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<String> for VarList {
    fn set(&mut self, index: usize, value: String) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeStr;
        self.data_stack_[index].data_ = AnyData::VStr(value);
    }
}

impl Put<String> for VarList {
    fn put(&mut self, value: String) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeStr,
            data_: AnyData::VStr(value),
        };
        self.data_stack_.push(var);
        self
    }
}

impl Init<Vec<ObjId>> for VarList {
    fn init(vec: Vec<ObjId>) -> VarList {
        let mut var_list = VarList::new();
        for value in vec.into_iter() {
            var_list = var_list + value;
        }
        var_list
    }
}

impl Add<ObjId> for VarList {
    type Output = VarList;

    fn add(self, value: ObjId) -> Self::Output {
        let mut temp = self.clone();
        let var = ValueData {
            type_: ValueType::ValueTypeObj,
            data_: AnyData::VObj(value),
        };
        temp.data_stack_.push(var);
        temp
    }
}

impl Get<ObjId> for VarList {
    fn get(&self, index: usize) -> ObjId {
        if index >= self.data_stack_.len() {
            return ObjId::new_null();
        }

        match self.data_stack_[index].data_ {
            AnyData::VObj(value) => value,
            _ => panic!("varList get type error!"),
        }
    }
}

impl Set<ObjId> for VarList {
    fn set(&mut self, index: usize, value: ObjId) {
        if index >= self.data_stack_.len() {
            return;
        }
        self.data_stack_[index].type_ = ValueType::ValueTypeObj;
        self.data_stack_[index].data_ = AnyData::VObj(value);
    }
}

impl Put<ObjId> for VarList {
    fn put(&mut self, value: ObjId) -> &mut Self {
        let var = ValueData {
            type_: ValueType::ValueTypeObj,
            data_: AnyData::VObj(value),
        };
        self.data_stack_.push(var);
        self
    }
}
