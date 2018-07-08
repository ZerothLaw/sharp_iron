//
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ptr;

//3rd party

//self
use clr::app_domain::{AppDomain};
use clr::c_api::BString;
use clr::error::*;

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

    pub fn app_domain(&self, name: &str) -> Result<AppDomain, ClrError>{
        let mut app_domain: *mut _AppDomain = ptr::null_mut();
        let bs_name = BString::from_str(name);
        let sys = unsafe { bs_name.as_sys() };
        let hr = unsafe {
            (*self.ptr).create_app_domain(sys, &mut app_domain)
        };

        match hr {
            0 => AppDomain::new(app_domain), 
            _ => Err(ClrError::inner_call(ErrorSource::CAppDomainManager(hr)))
        }
    }
}