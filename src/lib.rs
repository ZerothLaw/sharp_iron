extern crate libc;
extern crate winapi;
use winapi::ctypes::{c_void, c_char};
use winapi::shared::winerror::HRESULT;
use std::ffi::CString;

#[repr(C)]
struct CAPIResult {
	hr: HRESULT, 
	ok: bool, 
	c_ptr: *mut c_void
}

#[link(name="clr_c_api", kind="static")]
extern "C" {
	fn CLRMetaHost_new() -> CAPIResult;
	fn CLRMetaHost_get_runtime(host: *mut c_void, version: *const c_char) -> CAPIResult; 
	fn CLRRuntimeInfo_is_loadable(runtime_info: *mut c_void) -> bool;
	fn CLRRuntimeInfo_get_clr_runtime(runtime_info: *mut c_void) -> CAPIResult;
	fn CLRRuntimeHost_start(clr_host: *mut c_void) -> bool;
	fn CLRRuntimeHost_load_assembly(clr_info: *mut c_void, clr_host: *mut c_void, assembly_name: *const c_char) -> CAPIResult;
	fn CLRRuntimeHost_stop(clr_host: *mut c_void) -> CAPIResult;
	fn Assembly_release(assembly: *mut c_void) -> CAPIResult;
}

#[repr(C)]
pub struct CLRMetaHost {
	internal_ptr: *mut c_void 
}


impl CLRMetaHost {
	pub fn new() -> CLRMetaHost {
		unsafe {
			let res = CLRMetaHost_new();
			let ptr = match res.ok {
				true => res.c_ptr, 
				false => { panic!("CLRMetaHost::new call failed with HRESULT: {:?}", res.hr); }
			};
			assert!(!ptr.is_null());
			CLRMetaHost{ internal_ptr: ptr }
		}
	}
	
	pub fn get_runtime_info(&self, version: &str) -> CLRRuntimeInfo {
		assert!(!self.is_null());
		unsafe {
			let cs_version = CString::new(version).unwrap();
			let res = CLRMetaHost_get_runtime(self.internal_ptr, cs_version.as_ptr());
			let ptr = match res.ok {
				true => res.c_ptr, 
				false => {panic!("get_runtime_info call failed with HRESULT: {:?}", res.hr);}
			};
			assert!(!ptr.is_null());
			CLRRuntimeInfo::new(ptr)
		}
	}
	
	pub fn is_null(&self) -> bool {
		self.internal_ptr.is_null()
	}
}

#[repr(C)]
pub struct Assembly(*mut c_void);

impl Assembly {
	fn new(c_ptr: *mut c_void) -> Assembly {
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

#[repr(C)]
pub struct CLRRuntimeInfo {
	internal_ptr: *mut c_void
}

impl CLRRuntimeInfo {
	pub fn new(ptr: *mut c_void) -> CLRRuntimeInfo {
		CLRRuntimeInfo {
			internal_ptr: ptr
		}
	}
	pub fn is_loadable(&self) -> bool {
		unsafe {
			CLRRuntimeInfo_is_loadable(self.internal_ptr)
		}
	}
	
	pub fn get_clr_host(&self) -> Result<CLRRuntimeHost, HRESULT> {
		let res = unsafe {
			CLRRuntimeInfo_get_clr_runtime(self.internal_ptr)
		};
		match res.ok {
			true => Ok(CLRRuntimeHost{internal_ptr: res.c_ptr, started: false, assemblies: Vec::new()}), 
			false => Err(res.hr)
		}
	}	
	
	pub fn is_null(&self) -> bool {
		self.internal_ptr.is_null()
	}
}

#[repr(C)]
pub struct CLRRuntimeHost {
	internal_ptr: *mut c_void,
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
		let res = unsafe {
			let cs_assembly_name = CString::new(assembly_name).unwrap();
			CLRRuntimeHost_load_assembly(runtime_info.internal_ptr, self.internal_ptr, cs_assembly_name.as_ptr())
		};
		//print!("HRESULT is {:?}, ptr is {:?}", res.hr, res.c_ptr);
		if res.ok {
			self.assemblies.push(Assembly::new(res.c_ptr));
		}
		return res.ok;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn metahost() { 
		let host = CLRMetaHost::new();
		assert_eq!(host.is_null(), false);
	}
	
	#[test]
	fn runtime_info() {
		let host = CLRMetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		assert_eq!(runtime_info.is_null(), false);
		assert_eq!(runtime_info.is_loadable(), true);
	}
	
	#[test]
	fn get_clr_host() {
		let host = CLRMetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		assert_eq!(clr_host.is_null(), false);
	}
	
	#[test]
	fn start_runtime() { 
		let host = CLRMetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let mut clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		assert_eq!(clr_host.start(), true);
	}
	//TestAssembly, Version=1.0.0.0, Culture=neutral, PublicKeyToken=c97610437c81cba6
	//mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089
	#[test]
	fn load_assembly() {
		let host = CLRMetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let mut clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		let loaded_assembly = clr_host.load_assembly(runtime_info, "mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089");
		assert_eq!(loaded_assembly, true);
	}
}
