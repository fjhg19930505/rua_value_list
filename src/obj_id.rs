use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct ObjId {
    ident_: u32,
    serial_: u32,
}

impl ObjId {
    pub fn is_null(&self) -> bool {
        self.ident_ == 0 && self.serial_ == 0
    }

    pub fn equal_to(&self, other: &ObjId) -> bool {
        self.ident_ == other.ident_ && self.serial_ == other.serial_
    }

    pub fn new_null() -> ObjId {
        ObjId {
            ident_: 0,
            serial_: 0,
        }
    }
}

impl From<String> for ObjId {
    fn from(str_obj: String) -> Self {
        let arr_obj = str_obj.split("-");
        let vec = arr_obj.collect::<Vec<&str>>();
        if vec.len() != 2 {
            return ObjId::new_null();
        }

        let ident = u32::from_str(vec[0]).ok().unwrap();
        let serial = u32::from_str(vec[1]).ok().unwrap();
        ObjId {
            ident_: ident,
            serial_: serial,
        }
    }
}
