extern crate rustc_serialize;

mod macros;
pub mod config;
pub mod error;

pub mod obj_id;
pub mod value;
pub mod var;
pub mod var_list;

pub use config::Config;
pub use error::{ErrorKind, RpError, RpResult};
pub use obj_id::ObjId;
pub use value::*;
pub use var_list::*;
