//
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ptr;

//3rd party

use winapi::shared::winerror::{HRESULT};

//self
use clr::app_domain::{AppDomain};
use clr::c_api::{ClrArray};

use clr::bindings::{_AppDomain, IRustHostControl, ICustomAppDomainManager};

//body



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
        let sa_name = ClrArray::new(String::from(name));
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
}