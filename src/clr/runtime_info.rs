//runtime_info.rs
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
use clr::c_api::GetCurrentProcess;
use clr::runtime_host::{CLSID_CLRRuntimeHost, IID_ICLRRuntimeHost, ICLRRuntimeHost, RuntimeHost};

RIDL!{#[uuid(0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91)]
interface ICLRRuntimeInfo(ICLRRuntimeInfoVtbl): IUnknown(IUnknownVtbl) {
	fn GetVersionString(
		pwzBuffer: LPWSTR, 
		pcchBuffer: *mut DWORD,
	) -> HRESULT,
	
	fn GetRuntimeDirectory(
		pwzBuffer: LPWSTR, 
		pcchBuffer: *mut DWORD,
	) -> HRESULT,
	
	fn IsLoaded(
		hndProcess: HANDLE, 
		pbLoaded: *mut bool,
	) -> HRESULT,
	
	fn LoadErrorString(
		iResourceID: UINT, 
		pwzBuffer: LPWSTR, 
		pcchBuffer: *mut DWORD,
		iLocaleID: c_long,
	) -> HRESULT,
	
	fn LoadLibrary(
		pwzDllName: LPCWSTR, 
		phndModule: *mut HMODULE,
	) -> HRESULT,
	
	fn GetProcAddress(
		pszProcName: LPCSTR, 
		ppProc: *mut *mut LPVOID,
	) -> HRESULT,
	
	fn GetInterface(
		rclsid: REFCLSID, 
		riid: REFIID, 
		ppUnk: *mut LPVOID,
	) -> HRESULT,
	
	fn IsLoadable(
		pbLoadable: *mut bool,
	) -> HRESULT,
	
	fn SetDefaultStartupFlags(
		dwStartupFlags: DWORD, 
		pwzHostConfigFile: LPCWSTR,
	) -> HRESULT,
	
	fn GetDefaultStartupFlags(
		pdwStartupFlags: *mut DWORD, 
		pwzHostConfigFile: LPWSTR, 
		pcchHostConfigFile: *mut DWORD, 
	) -> HRESULT,
	
	fn BindAsLegacyV2Runtime() -> HRESULT, 

	fn IsStarted(
		pbStarted: *mut bool, 
		pdwStartupFlags: *mut DWORD,
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
				let _hr = (*self.ptr).IsLoadable(&mut res);
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
				let _hr = (*self.ptr).IsLoaded(handle, &mut res);
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
				let _hr = (*self.ptr).IsStarted(&mut res, flags);
				res 
			};
		}
		self.started
	}
	
	pub fn get_clr_host(&self) -> Result<RuntimeHost, HRESULT> {
		let res = unsafe {
			let mut out_ptr: *mut ICLRRuntimeHost = ptr::null_mut();
			coerce_pointer!(out_ptr, *mut LPVOID, ptr2);
			let _hr = (*self.ptr).GetInterface(&CLSID_CLRRuntimeHost, &IID_ICLRRuntimeHost, ptr2);
			out_ptr
		};
		Ok(RuntimeHost::new(res))
	}	
	
	// pub fn get_error_string(&self, hr: HRESULT) -> Result<WideString, String> {
	// 	let result = unsafe {CLRRuntimeInfo_load_error_string(self.internal_ptr, hr) };
	// 	match result.ok {
	// 		true => {
	// 			Ok( unsafe {WideString::from_ptr(result.c_ptr.bstr, result.hr as usize) })
	// 		}, 
	// 		false => Err("Couldn't load the resource string. Check that you have the correct library or DLL loaded into memory first.".to_string())
	// 	}
	// }
	
	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}
}