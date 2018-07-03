//runtime_info.rs
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ptr;

//3rd party
//use widestring::WideString;

use winapi::ctypes::{c_long};

use winapi::shared::guiddef::{REFIID, REFCLSID};
use winapi::shared::minwindef::{DWORD, HMODULE, LPVOID, UINT};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::winerror::HRESULT;

use winapi::um::winnt::{LPCSTR, LPCWSTR, LPWSTR};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};


//self

//self.structs
use clr::c_api::{GetCurrentProcess};
use clr::runtime_host::{CLSID_CLRRUNTIME_HOST, IID_ICLRRUNTIME_HOST, ICLRRuntimeHost, RuntimeHost};

RIDL!{#[uuid(0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91)]
interface ICLRRuntimeInfo(ICLRRuntimeInfoVtbl): IUnknown(IUnknownVtbl) {
	fn get_version_string(
		pwz_buffer: LPWSTR, 
		pcch_buffer: *mut DWORD,
	) -> HRESULT,
	
	fn get_runtime_directory(
		pwz_buffer: LPWSTR, 
		pcch_buffer: *mut DWORD,
	) -> HRESULT,
	
	fn is_loaded(
		hnd_process: HANDLE, 
		pb_loaded: *mut bool,
	) -> HRESULT,
	
	fn load_error_string(
		i_resource_id: UINT, 
		pwz_buffer: LPWSTR, 
		pcch_buffer: *mut DWORD,
		i_locale_id: c_long,
	) -> HRESULT,
	
	fn load_library(
		pwz_dll_name: LPCWSTR, 
		phnd_module: *mut HMODULE,
	) -> HRESULT,
	
	fn get_proc_address(
		psz_proc_name: LPCSTR, 
		pp_proc: *mut *mut LPVOID,
	) -> HRESULT,
	
	fn get_interface(
		rclsid: REFCLSID, 
		riid: REFIID, 
		pp_unk: *mut LPVOID,
	) -> HRESULT,
	
	fn is_loadable(
		pb_loadable: *mut bool,
	) -> HRESULT,
	
	fn set_default_startup_flags(
		dw_startup_flags: DWORD, 
		pwz_host_config_file: LPCWSTR,
	) -> HRESULT,
	
	fn get_default_startup_flags(
		pdw_startup_flags: *mut DWORD, 
		pwz_host_config_file: LPWSTR, 
		pcch_host_config_file: *mut DWORD, 
	) -> HRESULT,
	
	fn bind_as_legacy_v2_runtime() -> HRESULT, 

	fn is_started(
		pb_started: *mut bool, 
		pdw_startup_flags: *mut DWORD,
	) -> HRESULT,
}}


#[repr(C)]
pub struct RuntimeInfo {
	ptr: *mut ICLRRuntimeInfo,
	loaded: bool, 
	loadable: bool, 
	started: bool
}

impl RuntimeInfo {
	pub fn new(ptr: *mut ICLRRuntimeInfo) -> RuntimeInfo {
		if ptr.is_null() { panic!("Invalid/null ptr passed to RuntimeInfo::new");}
		RuntimeInfo {
			ptr: ptr, 
			loaded: false, 
			loadable: false, 
			started: false
		}
	}
	pub fn is_loadable(&mut self) -> bool {
		if !self.loadable {
			self.loadable = unsafe {
				let mut res = false;
				let _hr = (*self.ptr).is_loadable(&mut res);
				res
			};
		}
		self.loadable
	}
	
	pub fn is_loaded(&mut self) -> bool {
		if !self.loaded {
			let mut res = false;
			self.loaded = unsafe { 
				let handle = GetCurrentProcess();
				let _hr = (*self.ptr).is_loaded(handle, &mut res);
				res
			 };
		}
		self.loaded
	}
	
	pub fn is_started(&mut self) -> bool {
		if !self.started {
			let mut res = false;
			self.started = unsafe {
				let flags: *mut DWORD = ptr::null_mut();
				let _hr = (*self.ptr).is_started(&mut res, flags);
				res 
			};
		}
		self.started
	}
	
	pub fn get_clr_host(&self) -> Result<RuntimeHost, HRESULT> {
		let res = unsafe {
			let mut out_ptr: *mut ICLRRuntimeHost = ptr::null_mut();
			coerce_pointer!(out_ptr, *mut LPVOID, ptr2);
			let _hr = (*self.ptr).get_interface(&CLSID_CLRRUNTIME_HOST, &IID_ICLRRUNTIME_HOST, ptr2);
			out_ptr
		};
		Ok(RuntimeHost::new(res))
	}
	
	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}
}