//runtime_host.rs

//std
//use std::ffi::CString;
use std::ptr;

//3rd party
use widestring::WideCString;

use winapi::ctypes::{c_void, c_int};

use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::{HRESULT};

use winapi::um::winnt::{LPCWSTR};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

//self
use clr::misc::{ICLRControl};
use clr::host_control::{IHostControl, IRustHostControl, RustHostControl};
use clr::c_api::RustHostControl_new;


DEFINE_GUID!{IID_ICLRRuntimeHost, 
	0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

DEFINE_GUID!{CLSID_CLRRuntimeHost, 
	0x90F1A06E, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

/*
typedef HRESULT ( __stdcall *FExecuteInAppDomainCallback )( 
    void *cookie);
*/
type FExecuteInAppDomainCallback = Fn(*mut c_void)->HRESULT;

RIDL!{#[uuid(0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02)]
interface ICLRRuntimeHost(ICLRRuntimeHostVtbl): IUnknown(IUnknownVtbl) {
	fn Start() -> HRESULT,
	fn Stop() -> HRESULT,
	fn SetHostControl(
		pHostControl: *mut IHostControl,
	) -> HRESULT, 
	fn GetCLRControl(
		pCLRControl: *mut *mut ICLRControl, 
	) -> HRESULT,
	fn UnloadAppDomain(
		dwAppDomainId: DWORD, 
		fWaitUntilDone: bool,
	) -> HRESULT,
	fn ExecuteInAppDomain(
		dwAppDomainId: DWORD, 
		pCallback: *mut FExecuteInAppDomainCallback, 
		cookie: *mut c_void,
	) -> HRESULT,
	fn GetCurrentAppDomainId(
		pdwAppDomainId: *mut DWORD,
	) -> HRESULT,
	fn ExecuteApplication(
		pwzAppFullName: LPCWSTR, 
		dwManifestPaths: DWORD, 
		ppwzManifestPaths: *mut LPCWSTR, 
		dwActivationData: DWORD, 
		ppwzActivationData: *mut LPCWSTR, 
		pReturnValue: *mut c_int, 
	) -> HRESULT,
	fn ExecuteInDefaultAppDomain(
		pwzAssemblyPath: LPCWSTR, 
		pwzTypeName: LPCWSTR, 
		pwzArgument: LPCWSTR, 
		pReturnValue: *mut DWORD,
	) -> HRESULT,
}}

#[repr(C)]
pub struct RuntimeHost {
	pub ptr: *mut ICLRRuntimeHost,
	started: bool,
	host_control: Option<*mut IRustHostControl>,
}

impl Drop for RuntimeHost {
	fn drop(&mut self) {
		if self.started {
			unsafe {
				(*self.ptr).Stop();
			}
		}
	}
}

impl RuntimeHost {
	pub fn new(c_ptr: *mut ICLRRuntimeHost) -> RuntimeHost {
		if c_ptr.is_null() {panic!("Passed a null ptr of ICLRRuntimeHost to RuntimeHost::new");}
		RuntimeHost{ptr: c_ptr, started: false, host_control: None}
	}
	pub fn start(&mut self) -> bool {
		if !self.started {
			self.started = unsafe {
				//need to implement host control
				let host_control: *mut IRustHostControl = RustHostControl_new() ;
				let _hr = (*self.ptr).SetHostControl(host_control as *mut IHostControl);
				self.register_app_domain_manager_type();
				self.host_control = Some(host_control);
				let hr = (*self.ptr).Start();
				println!("HRESULT={:x}", hr);
				hr == 0
			};
		}
		self.started
	}
	
	unsafe fn register_app_domain_manager_type(&mut self) {
		let mut p_control: *mut ICLRControl = ptr::null_mut();
		coerce_pointer!(p_control, *mut *mut ICLRControl, ptr2);
		let hr = (*self.ptr).GetCLRControl(ptr2);
		println!("HRESULT={:x}", hr);
		let assembly_name = WideCString::from_str("RustAppDomainManager, Version=1.0.0.0, Culture=neutral, PublicKeyToken=a1db4d7bbefc8ca0, processorArchitecture=MSIL").unwrap();
		let type_name = WideCString::from_str("RustAppDomainManager.RustAppDomainManager").unwrap();
		let hr = (*p_control).SetAppDomainManagerType(assembly_name.as_ptr(), type_name.as_ptr());
		println!("HRESULT={:x}", hr);
	}

	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}

	pub fn get_current_app_domain_id(&self) -> u32 {
		let mut id: DWORD = 0;
		unsafe {
			(*self.ptr).GetCurrentAppDomainId(&mut id);
			id
		}
	}

	pub fn get_host_control(&self) -> RustHostControl {
		RustHostControl::new(self.host_control.unwrap())
	}
	
	// pub fn load_assembly(&mut self, runtime_info: RuntimeInfo, assembly_name: &str) -> bool {
	// 	if !self.started {
	// 		self.start();
	// 	}
	// 	let cs_assembly_name = CString::new(assembly_name).unwrap();
	// 	let res = unsafe {
	// 		CLRRuntimeHost_load_assembly(self.internal_ptr, runtime_info.internal_ptr, cs_assembly_name.as_ptr())
	// 	};

	// 	return res.ok;
	// }
}