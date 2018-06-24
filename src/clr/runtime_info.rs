//runtime_info.rs
//std

//3rd party
use widestring::WideString;
use winapi::ctypes::c_void;
use winapi::shared::winerror::HRESULT;
use winapi::shared::ntdef::HANDLE;

//self
use clr::c_api::CLRRuntimeInfo_is_loadable;
use clr::c_api::CLRRuntimeInfo_is_loaded;
use clr::c_api::CLRRuntimeInfo_is_started;
use clr::c_api::CLRRuntimeInfo_is_loaded_from_handle;
use clr::c_api::CLRRuntimeInfo_get_clr_runtime;
use clr::c_api::CLRRuntimeInfo_load_error_string;

//self.structs
use clr::runtime_host::RuntimeHost;


#[repr(C)]
pub struct RuntimeInfo {
	pub internal_ptr: *mut c_void,
	loaded: bool, 
	loadable: bool, 
	started: bool
}

impl RuntimeInfo {
	pub fn new(ptr: *mut c_void) -> RuntimeInfo {
		RuntimeInfo {
			internal_ptr: ptr, 
			loaded: false, 
			loadable: false, 
			started: false
		}
	}
	pub fn is_loadable(&mut self) -> bool {
		if !self.loadable {
			self.loadable = unsafe { CLRRuntimeInfo_is_loadable(self.internal_ptr)};
		}
		self.loadable
	}
	
	pub fn is_loaded(&mut self) -> bool {
		if !self.loaded {
			self.loaded = unsafe { CLRRuntimeInfo_is_loaded(self.internal_ptr) };
		}
		self.loaded
	}
	
	pub fn is_started(&mut self) -> bool {
		if !self.started {
			self.started = unsafe {CLRRuntimeInfo_is_started(self.internal_ptr)};
		}
		self.started
	}
	
	pub fn is_loaded_from_handle(&mut self, process_handle: HANDLE) -> bool {
		unsafe {
			CLRRuntimeInfo_is_loaded_from_handle(self.internal_ptr, process_handle)
		}
	}
	
	pub fn get_clr_host(&self) -> Result<RuntimeHost, HRESULT> {
		let res = unsafe {
			CLRRuntimeInfo_get_clr_runtime(self.internal_ptr)
		};
		match res.ok {
			true => Ok(RuntimeHost::new(res.c_ptr)), 
			false => Err(res.hr)
		}
	}	
	
	pub fn get_error_string(&self, hr: HRESULT) -> Result<WideString, String> {
		let result = unsafe {CLRRuntimeInfo_load_error_string(self.internal_ptr, hr) };
		match result.ok {
			true => {
				Ok( unsafe {WideString::from_ptr(result.ws_ptr, result.hr as usize) })
			}, 
			false => Err("Couldn't load the resource string. Check that you have the correct library or DLL loaded into memory first.".to_string())
		}
	}
	
	pub fn is_null(&self) -> bool {
		self.internal_ptr.is_null()
	}
}