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


DEFINE_GUID!{IID_ICLRRUNTIME_HOST, 
	0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

DEFINE_GUID!{CLSID_CLRRUNTIME_HOST, 
	0x90F1A06E, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

/*
typedef HRESULT ( __stdcall *FExecuteInAppDomainCallback )( 
    void *cookie);
*/
type FExecuteInAppDomainCallback = Fn(*mut c_void)->HRESULT;

RIDL!{#[uuid(0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02)]
interface ICLRRuntimeHost(ICLRRuntimeHostVtbl): IUnknown(IUnknownVtbl) {
	fn start() -> HRESULT,
	fn stop() -> HRESULT,
	fn set_host_control(
		host_control: *mut IHostControl,
	) -> HRESULT, 
	fn get_clr_control(
		clr_control: *mut *mut ICLRControl, 
	) -> HRESULT,
	fn unload_app_domain(
		app_domain_id: DWORD, 
		wait_until_done: bool,
	) -> HRESULT,
	fn execute_in_app_domain(
		app_domain_id: DWORD, 
		callback: *mut FExecuteInAppDomainCallback, 
		cookie: *mut c_void,
	) -> HRESULT,
	fn get_current_app_domain_id(
		app_domain_id: *mut DWORD,
	) -> HRESULT,
	fn execute_application(
		app_full_name: LPCWSTR, 
		dw_manifest_paths: DWORD, 
		manifest_paths: *mut LPCWSTR, 
		dw_activation_data: DWORD, 
		activation_data: *mut LPCWSTR, 
		return_value: *mut c_int, 
	) -> HRESULT,
	fn execute_in_default_app_domain(
		assembly_path: LPCWSTR, 
		type_name: LPCWSTR, 
		argument: LPCWSTR, 
		return_value: *mut DWORD,
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
				(*self.ptr).stop();
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
				let _hr = (*self.ptr).set_host_control(host_control as *mut IHostControl);
				self.register_app_domain_manager_type();
				self.host_control = Some(host_control);
				let hr = (*self.ptr).start();
				println!("HRESULT={:x}", hr);
				hr == 0
			};
		}
		self.started
	}
	
	unsafe fn register_app_domain_manager_type(&mut self) {
		let mut p_control: *mut ICLRControl = ptr::null_mut();
		coerce_pointer!(p_control, *mut *mut ICLRControl, ptr2);
		let hr = (*self.ptr).get_clr_control(ptr2);
		println!("HRESULT={:x}", hr);
		let assembly_name = WideCString::from_str("RustAppDomainManager, Version=1.0.0.0, Culture=neutral, PublicKeyToken=a1db4d7bbefc8ca0, processorArchitecture=MSIL").unwrap();
		let type_name = WideCString::from_str("RustAppDomainManager.RustAppDomainManager").unwrap();
		let hr = (*p_control).set_app_domain_manager_type(assembly_name.as_ptr(), type_name.as_ptr());
		println!("HRESULT={:x}", hr);
	}

	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}

	pub fn get_current_app_domain_id(&self) -> u32 {
		let mut id: DWORD = 0;
		unsafe {
			(*self.ptr).get_current_app_domain_id(&mut id);
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