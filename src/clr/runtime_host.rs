//runtime_host.rs

//std
use std::ffi::CString;

//3rd party
use winapi::ctypes::c_void;

//self
use clr::assembly::Assembly;
use clr::c_api::CLRRuntimeHost_stop;
use clr::c_api::CLRRuntimeHost_start;
use clr::c_api::CLRRuntimeHost_load_assembly;
use clr::runtime_info::CLRRuntimeInfo;

#[repr(C)]
pub struct CLRRuntimeHost {
	pub internal_ptr: *mut c_void,
	started: bool, 
	assemblies: Vec<Assembly>
}

impl Drop for CLRRuntimeHost {
	fn drop(&mut self) {
		if self.started {
			unsafe {
				CLRRuntimeHost_stop(self.internal_ptr);
			}
		}
	}
}

impl CLRRuntimeHost {
	pub fn new(c_ptr: *mut c_void) -> CLRRuntimeHost {
		CLRRuntimeHost{internal_ptr: c_ptr, started: false, assemblies: Vec::new()}
	}
	pub fn start(&mut self) -> bool {
		if !self.started {
			self.started = unsafe {
				CLRRuntimeHost_start(self.internal_ptr)
			};
		}
		self.started
	}
	
	pub fn is_null(&self) -> bool {
		self.internal_ptr.is_null()
	}
	
	pub fn load_assembly(&mut self, runtime_info: CLRRuntimeInfo, assembly_name: &str) -> bool {
		if !self.started {
			self.start();
		}
		let cs_assembly_name = CString::new(assembly_name).unwrap();
		let res = unsafe {
			CLRRuntimeHost_load_assembly(self.internal_ptr, runtime_info.internal_ptr, cs_assembly_name.as_ptr())
		};
		if res.ok {
			self.assemblies.push(Assembly::new(res.c_ptr));
		}
		return res.ok;
	}
}