//
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ptr;
use std::mem;

//3rd party
use widestring::WideCString;

use winapi::ctypes::{c_void};

use winapi::shared::guiddef::{REFIID};
use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::combaseapi::{CoTaskMemAlloc, CoTaskMemFree};

//self
use clr::app_domain::{_AppDomain, AppDomain};
use clr::c_api::{ClrArray};
use clr::assembly::{Assembly, _Assembly};

//body

RIDL!{#[uuid(0xB47320A6, 0x6265, 0x4C34, 0x90, 0xAC, 0x3F, 0xF2, 0xA9, 0x09, 0x68, 0x6C)]
interface ICustomAppDomainManager(ICustomAppDomainManagerVtbl): IDispatch(IDispatchVtbl){
    fn get_app_domain(
        friendly_name: *mut SAFEARRAY,
        app_domain: *mut *mut _AppDomain,
    ) -> HRESULT,
    fn create_app_domain(
        name: *mut SAFEARRAY,
        app_domain: *mut *mut _AppDomain,
    )-> HRESULT,
    fn LoadAssembly(
        name: *mut SAFEARRAY, //passing a safearray to byte[]
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn TestingCall(
        sTest: *mut u16, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x02CA073C, 0x7079, 0x4860, 0x88, 0x0A, 0xC2, 0xF7, 0xA4, 0x49, 0xC9, 0x91)]
interface IHostControl(IHostControlVtbl): IUnknown(IUnknownVtbl){
	fn get_host_manager(
		riid: REFIID, 
		pp_object: *mut *mut c_void,
	) -> HRESULT,
	fn set_app_domain_manager(
		dw_app_domain_id: DWORD, 
		p_unk_app_domain_manager: *mut IUnknown, 
	) -> HRESULT,
}}

RIDL!{#[uuid(0x1e20d486, 0x67c7, 0x4cd6, 0xb5, 0x6b, 0x41, 0xd2, 0x29, 0x7d, 0x5b, 0x2f)]
interface IRustHostControl(IRustHostControlVtbl) : IHostControl(IHostControlVtbl){
    fn  domain_manager() -> *mut ICustomAppDomainManager,
}}

pub struct RustHostControl{
    ptr: *mut IRustHostControl,
}

impl RustHostControl {
    pub fn new(ptr: *mut IRustHostControl) -> RustHostControl {
        RustHostControl {
            ptr: ptr
        }
    }

    pub fn get_domain_manager(&self) -> RustDomainManager{
        let ptr = unsafe {
            (*self.ptr).domain_manager()
        };
        RustDomainManager::new(ptr)
    }
}


pub struct RustDomainManager {
    ptr: *mut ICustomAppDomainManager
}

impl RustDomainManager {
    fn new(in_ptr: *mut ICustomAppDomainManager) -> RustDomainManager {
        RustDomainManager { ptr: in_ptr }
    }

    pub fn app_domain(&self, name: &str) -> Result<AppDomain, HRESULT>{
        let mut app_domain: *mut _AppDomain = ptr::null_mut();
        let sa_name = ClrArray::new(name);
        match sa_name.to_safearray() {
            Ok(psa) => {
                let _hr = unsafe {
                    (*self.ptr).create_app_domain(psa, &mut app_domain)
                };
                Ok(AppDomain::new(app_domain))
            }, 
            Err(hr) => {
                Err(hr)
            }
        }
    }

    pub fn load_assembly(&self, name: &str) -> Result<Assembly, HRESULT> {

        let mut res: *mut _Assembly = ptr::null_mut();
        let safe = ClrArray::new(name);
        match safe.to_safearray() {
            Ok(psa) => {
                let hr = unsafe {
                    (*self.ptr).LoadAssembly(psa, &mut res)
                };

                match hr {
                    0 => Ok(Assembly::new(res)), 
                    _ => Err(hr)
                }
            }, 
            Err(hr) => Err(hr)
        }
    }
    pub fn test_call(&self, val: &str) {
        let s_len = val.len();
        println!("s_len = {}", s_len);
        let ws = WideCString::from_str(val).unwrap();
        unsafe {
            let raw = ws.into_raw();
            let mem_size = (mem::size_of::<u16>() * s_len) + 4 + 2; //also need length prefix + null terminator
            let contents_size = mem_size - 4 - 2;
            println!("contents_size: {}", contents_size);
            println!("mem_size: {}", mem_size);
            //BSTR is a length prefixed double-byte string
            let clr_raw: *mut u16 = CoTaskMemAlloc(mem_size) as *mut u16; //SIZE_T => ULONG_PTR => usize
            //copy bytes from raw to clr_raw
            let n: *const u16 = &contents_size as *const usize as *const u64 as *const u32 as *const u16;
            //println!("n = {}", *n);
            n.copy_to(clr_raw, 2);
            let clr_raw = clr_raw.add(2);
            raw.copy_to(clr_raw, contents_size);
            clr_raw.add(contents_size).write_bytes(0, 1);

            let hr = (*self.ptr).TestingCall(clr_raw);
            let _ws = WideCString::from_raw(raw); //retake raw pointer to prevent memory leaks
            println!("HRESULT = {:x}", hr);
        };
    }
}