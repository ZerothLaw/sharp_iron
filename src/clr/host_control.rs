//

//std
use std::ptr;

//3rd party
use winapi::ctypes::{c_void};

use winapi::shared::guiddef::{REFIID};
use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::BSTR;

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

//self
use clr::misc::_AppDomain;

//body

RIDL!{#[uuid(0xB47320A6, 0x6265, 0x4C34, 0x90, 0xAC, 0x3F, 0xF2, 0xA9, 0x09, 0x68, 0x6C)]
interface ICustomAppDomainManager(ICustomAppDomainManagerVtbl): IDispatch(IDispatchVtbl){
    fn GetAppDomain(
        friendly_name: BSTR,
        app_domain: *mut *mut _AppDomain,
    ) -> HRESULT,
    fn CreateAppDomain(
        app_domain: *mut *mut _AppDomain,
    )-> HRESULT,
}}

RIDL!{#[uuid(0x02CA073C, 0x7079, 0x4860, 0x88, 0x0A, 0xC2, 0xF7, 0xA4, 0x49, 0xC9, 0x91)]
interface IHostControl(IHostControlVtbl): IUnknown(IUnknownVtbl){
	fn GetHostManager(
		riid: REFIID, 
		ppObject: *mut *mut c_void,
	) -> HRESULT,
	fn SetAppDomainManager(
		dwAppDomainID: DWORD, 
		pUnkAppDomainManager: *mut IUnknown, 
	) -> HRESULT,
}}

RIDL!{#[uuid(0x1e20d486, 0x67c7, 0x4cd6, 0xb5, 0x6b, 0x41, 0xd2, 0x29, 0x7d, 0x5b, 0x2f)]
interface IRustHostControl(IRustHostControlVtbl) : IHostControl(IHostControlVtbl){
    fn  GetDomainManager() -> *mut ICustomAppDomainManager,
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
            (*self.ptr).GetDomainManager()
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

    pub fn app_domain(&self) -> *mut _AppDomain{
        unsafe {
            let mut app_domain: *mut _AppDomain = ptr::null_mut();
            let mut app_domain: *mut *mut _AppDomain = &mut app_domain;
            let hr = (*self.ptr).CreateAppDomain(app_domain);
            *app_domain
        }
    }
}