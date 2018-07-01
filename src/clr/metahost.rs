//clr meta host
//std
//use std::ffi::CString;
use std::ptr;

//3rd party
use widestring::WideCString;

use winapi::ctypes::{c_void};
use winapi::shared::guiddef::{REFIID};
use winapi::shared::minwindef::{DWORD,LPVOID};
use winapi::shared::winerror::{HRESULT};
use winapi::um::objidlbase::{IEnumUnknown};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HANDLE, LPCWSTR, LPWSTR};


//self
use clr::c_api::CLRCreateInstance;
use clr::runtime_info::{ICLRRuntimeInfo, RuntimeInfo};

RIDL!{#[uuid(0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16)]
interface ICLRMetaHost(ICLRMetaHostVtbl): IUnknown(IUnknownVtbl) {
	fn get_runtime(
		pwzVersion: LPCWSTR, 
		riid: REFIID, 
		ppRuntime: *mut LPVOID,
	) -> HRESULT, 
	fn GetVersionFromFile(
		pwzFilePath: LPCWSTR, 
		pwzBuffer: LPWSTR,
		pcchBuffer: *mut DWORD,
	) -> HRESULT, 
	fn EnumerateInstalledRuntimes(
		ppEnumerator: *mut *mut IEnumUnknown,
	) -> HRESULT,
	fn EnumerateLoadedRuntimes(
		hndProcess: HANDLE, 
		ppEnumerator: *mut *mut IEnumUnknown,
	) -> HRESULT,
	fn RequestRuntimeLoadedNotification(
		pCallbackFunction: *mut c_void,
	) -> HRESULT, 
	fn QueryLegacyV2RuntimeBinding(
		riid: REFIID, 
		ppUnk: *mut LPVOID,
	) -> HRESULT,
	fn ExitProcess(
		iExitCode: i32,
	) -> HRESULT,
}}

//body

DEFINE_GUID!{GUID_ICLRRuntimeInfo, 
	 0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91}

//0x9280188d, 0xe8e, 0x4867, 0xb3, 0xc, 0x7f, 0xa8, 0x38, 0x84, 0xe8, 0xde
DEFINE_GUID!{CLSID_CLRMetaHost, 
	0x9280188d, 0x0e8e, 0x4867, 0xb3, 0x0c, 0x7f, 0xa8, 0x38, 0x84, 0xe8, 0xde}

//EXTERN_GUID(IID_ICLRMetaHost, 0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16);
DEFINE_GUID!{IID_ICLRMetaHost, 
	0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16}

#[repr(C)]
pub struct MetaHost {
	ptr: *mut ICLRMetaHost
}

impl MetaHost {
	pub fn new() -> MetaHost {
		let res = unsafe {
			let mut p_host: *mut ICLRMetaHost = ptr::null_mut();
			coerce_pointer!(p_host, *mut LPVOID, ptr2);
			let _hr = CLRCreateInstance(&CLSID_CLRMetaHost, &IID_ICLRMetaHost, ptr2);
			p_host
		};
		
		MetaHost { ptr: res }
	}
	
	pub fn get_runtime_info(&self, version: &str) -> RuntimeInfo {
		assert!(!self.is_null());
		let ws_version = WideCString::from_str(version).unwrap();
		let ptr = unsafe {
			let mut in_ptr: *mut ICLRRuntimeInfo = ptr::null_mut();
			coerce_pointer!(in_ptr, *mut LPVOID, ptr2);
			let _hr = (*self.ptr).get_runtime(ws_version.as_ptr(), &GUID_ICLRRuntimeInfo, ptr2);
			in_ptr
		};

		RuntimeInfo::new(ptr)
	}
	
	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}
}

