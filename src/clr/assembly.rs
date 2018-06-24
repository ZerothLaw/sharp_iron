//assembly.rs
//std

//3rd party
use winapi::ctypes::c_void;

//self
use clr::c_api::Assembly_release;

#[repr(C)]
pub struct Assembly(*mut c_void);

impl Assembly {
	pub fn new(c_ptr: *mut c_void) -> Assembly {
		Assembly(c_ptr)
	}
}

impl Drop for Assembly {
	fn drop(&mut self) {
		unsafe {
			Assembly_release(self.0);
		}
	}
}
