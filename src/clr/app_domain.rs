//
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ptr;

//3rd party
use widestring::{WideCStr};

use winapi::shared::winerror::{HRESULT};

//self
use clr::assembly::{Assembly};

use clr::c_api::{BString};

use clr::bindings::{_AppDomain, _Assembly, _ObjectHandle};

#[derive(Debug)]
pub struct AppDomain<'a>{
    ptr: *mut _AppDomain,
    name: Option<String>,
    assemblies: Vec<&'a Assembly>
}

impl<'a> AppDomain<'a> {
    pub fn new(in_ptr: *mut _AppDomain) -> AppDomain<'a> {
        AppDomain {
            ptr: in_ptr, 
            name: None, 
            assemblies: Vec::new()
        }
    }

    pub fn name(&mut self) -> &str {
        if self.name.is_none() {
            let mut raw: *mut u16 = ptr::null_mut();
            let hr = unsafe {
                (*self.ptr).get_friendly_name(&mut raw)
            };
            match hr {
                0 => {
                    let new_ws = unsafe {WideCStr::from_ptr_str(raw)}; //borrow the string pointer, as string was allocated by the CLR/C++. 
                    let res = new_ws.to_string_lossy();
                    self.name = Some(res);
                }, 
                _ => {
                    panic!(format!("name() returned HR={:x}", hr));
                }
            }
        }
        match &self.name {
            Some(nm) => nm, 
            None => ""
        }
    }

    pub fn load_assembly(&self, name: &str) -> Result<Assembly, HRESULT> {
        let bs = BString::from_str(name);
        let mut asm_ptr: *mut _Assembly = ptr::null_mut();
        let hr = unsafe {
            let raw = bs.as_sys();
            (*self.ptr).load_2(raw, &mut asm_ptr)
        };
        match hr {
            0 => Ok(Assembly::new(asm_ptr)), 
            _ => Err(hr)
        }     
    }

    pub fn create_instance(&self, assembly_name: &str, type_name: &str) -> Result<*mut _ObjectHandle, HRESULT> {
        let bs_name = BString::from_str(assembly_name);
        let bs_type = BString::from_str(type_name);

        let mut obj_ptr: *mut _ObjectHandle = ptr::null_mut();
        let hr = unsafe {
            let raw_name = bs_name.as_sys();
            let raw_type = bs_type.as_sys();
            (*self.ptr).create_instance(raw_name, raw_type, &mut obj_ptr)
        };
        match hr {
            0 => Ok(obj_ptr), 
            _ => Err(hr)
        }
    }
} 


//body


