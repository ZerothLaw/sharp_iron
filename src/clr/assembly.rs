//assembly.rs

#![allow(dead_code)]
#![allow(non_snake_case)]
//std

//3rd party
use winapi::shared::wtypes::{VARTYPE, VT_UNKNOWN};

//self
use clr::type_::{Type};

use clr::c_api::ClrWrapper;

use clr::bindings::_Assembly;

//body
#[derive(Debug)]
pub struct Assembly {
	ptr: *mut _Assembly
}

impl Assembly {
	pub fn new(in_ptr: *mut _Assembly) -> Assembly{
		Assembly {
			ptr: in_ptr
		}
	}

	fn get_types(&mut self) -> Vec<Type> {
		// let type_ptrs = unsafe {
		// 	(*self.ptr).get_types()
		// };
		Vec::new()
	}
}

impl ClrWrapper for Assembly {
	type InnerPointer = _Assembly;

	fn into_raw(&self) -> *mut Self::InnerPointer {
		self.ptr
	}

	fn vartype() -> VARTYPE {
		VT_UNKNOWN as u16
	}
}
