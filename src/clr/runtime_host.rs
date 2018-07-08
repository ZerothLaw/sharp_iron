//runtime_host.rs
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
//use std::ffi::CString;
use std::ptr;

//3rd party
use widestring::WideCString;

use winapi::shared::minwindef::{DWORD};

//self
use clr::host_control::{RustHostControl};
use clr::c_api::RustHostControl_new;

use clr::bindings::ICLRControl;
use clr::bindings::ICLRRuntimeHost;
use clr::bindings::IHostControl;
use clr::bindings::IRustHostControl;

DEFINE_GUID!{IID_ICLRRUNTIME_HOST, 
	0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

DEFINE_GUID!{CLSID_CLRRUNTIME_HOST, 
	0x90F1A06E, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

#[repr(C)]
pub struct RuntimeHost {
	pub ptr: *mut ICLRRuntimeHost,
	started: bool,
	host_control: Option<RustHostControl>,
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
				self.host_control = Some(RustHostControl::new(host_control));
				let hr = (*self.ptr).start();
				println!("HRESULT={:x}a", hr);
				hr == 0
			};
		}
		self.started
	}
	
	unsafe fn register_app_domain_manager_type(&mut self) {
		let mut p_control: *mut ICLRControl = ptr::null_mut();
		coerce_pointer!(p_control, *mut *mut ICLRControl, ptr2);
		let hr = (*self.ptr).get_clr_control(ptr2);
		println!("HRESULT={:x}b", hr);
		let assembly_name = WideCString::from_str("RustAppDomainManager, Version=1.0.1.11, Culture=neutral, PublicKeyToken=a1db4d7bbefc8ca0, processorArchitecture=MSIL").unwrap();
		let type_name = WideCString::from_str("RustAppDomainManager.RustAppDomainManager").unwrap();
		let hr = (*p_control).set_app_domain_manager_type(assembly_name.as_ptr(), type_name.as_ptr());
		println!("HRESULT={:x}c", hr);
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

	pub fn get_host_control(&self) -> Option<&RustHostControl> {
		match &self.host_control {
			Some(hc) => Some(&hc),
			None => None
		}
	}
}