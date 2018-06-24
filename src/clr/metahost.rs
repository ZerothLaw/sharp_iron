//clr meta host
//std
use std::ffi::CString;

//3rd party
use winapi::ctypes::{c_void};

//self
// use clr::c_api::CAPIResult;
use clr::c_api::CLRMetaHost_new;
use clr::c_api::CLRMetaHost_get_runtime;
use clr::runtime_info::CLRRuntimeInfo;

//body
#[repr(C)]
pub struct CLRMetaHost {
	internal_ptr: *mut c_void 
}

impl CLRMetaHost {
	pub fn new() -> CLRMetaHost {
		let res = unsafe {CLRMetaHost_new()};
		
		let ptr = match res.ok {
			true => res.c_ptr, 
			false => { panic!("CLRMetaHost::new call failed with HRESULT: {:?}", res.hr); }
		};
		assert!(!ptr.is_null());
		CLRMetaHost{ internal_ptr: ptr }
		
	}
	
	pub fn get_runtime_info(&self, version: &str) -> CLRRuntimeInfo {
		assert!(!self.is_null());
		let cs_version = CString::new(version).unwrap();
		let res = unsafe { CLRMetaHost_get_runtime(self.internal_ptr, cs_version.as_ptr()) };
		let ptr = match res.ok {
			true => res.c_ptr, 
			false => {panic!("get_runtime_info call failed with HRESULT: {:?}", res.hr);}
		};
		assert!(!ptr.is_null());
		CLRRuntimeInfo::new(ptr)
	}
	
	pub fn is_null(&self) -> bool {
		self.internal_ptr.is_null()
	}
}

