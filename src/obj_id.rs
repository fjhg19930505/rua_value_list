#[derive(Debug, Copy, Clone)]
pub struct ObjId {
    ident_: u32,
    serial_: u32,
}

impl ObjId {
    pub fn is_null(&self) -> bool{
        self.ident_ == 0 && self.serial_ == 0
    }

    pub fn equal_to(&self, other: &ObjId) -> bool{
        self.ident_ == other.ident_ && self.serial_ == other.serial_
    }

    pub fn new_null() -> ObjId {
        ObjId{ident_: 0, serial_: 0}
    }

}