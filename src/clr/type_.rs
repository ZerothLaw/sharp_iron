//type_.rs
#![allow(dead_code)]
#![allow(non_snake_case)]
//std

//3rd party
use winapi::shared::wtypes::{VARTYPE, VT_UNKNOWN};

//self
use clr::c_api::{ClrWrapper};

use clr::bindings::_Type;

pub struct Type {
  ptr: *mut _Type
}

impl ClrWrapper for Type {
  type InnerPointer = _Type;
  
  fn into_raw(&self) -> *mut Self::InnerPointer {
    self.ptr
  }
	fn vartype() -> VARTYPE {
    VT_UNKNOWN as u16
  }

}
