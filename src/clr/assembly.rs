//assembly.rs
//std

//3rd party
use winapi::ctypes::c_void;
use winapi::shared::winerror::HRESULT;

//self
use clr::type_::Type_;
use clr::c_api::Assembly_release;
use clr::c_api::Assembly_get_type;


#[repr(C)]
pub struct Assembly(*mut c_void);

impl Assembly {
	pub fn new(c_ptr: *mut c_void) -> Assembly {
		Assembly(c_ptr)
	}
	
	pub fn type_(&self) -> Result<Type_, HRESULT> {
		return Err(1);
	}
}

impl Drop for Assembly {
	fn drop(&mut self) {
		unsafe {
			Assembly_release(self.0);
		}
	}
}
