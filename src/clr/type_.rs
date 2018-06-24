//type_.rs
//std

//3rd party
use winapi::ctypes::c_void;

//self
use clr::c_api::Type_call_static_method;
use clr::c_api::Type_create_instance;

#[repr(C)]
pub struct Type_ {
	internal_ptr: *mut c_void
}