//
#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ptr;

//3rd party
use widestring::{WideCStr};
use winapi::Interface;
use winapi::ctypes::{c_long, c_short};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

//self
use clr::assembly::{Assembly, _Assembly, _AssemblyName, AssemblyBuilderAccess};
use clr::type_::_Type;
use clr::misc::{_AssemblyLoadEventHandler, _Binder, _CultureInfo, _EventHandler, _Evidence, _PolicyLevel, _ResolveEventHandler, _UnhandledExceptionEventHandler, 
BindingFlags, IPrincipal};
use clr::c_api::{BString};
use clr::object_handle::{_ObjectHandle};

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
RIDL!{#[uuid(0xaf93163f, 0xc2f4, 0x3fab, 0x9f, 0xf1, 0x72, 0x8a, 0x7a, 0xaa, 0xd1, 0xcb)]
interface _CrossAppDomainDelegate(_CrossAppDomainDelegateVtbl) : IDispatch(IDispatchVtbl){

}}
#[repr(C)]
pub enum PrincipalPolicy
{
    UnauthenticatedPrincipal = 0,
    NoPrincipal = 1,
    WindowsPrincipal = 2
}
add_uuid!(PrincipalPolicy, 0x7d29bc4b, 0x8fbc, 0x38aa, 0x8b, 0x35, 0xed, 0x45, 0x39, 0xa1, 0xcf, 0x8e);

RIDL!{#[uuid(0xc2af4970, 0x4fb6, 0x319c, 0xa8, 0xaa, 0x06, 0x14, 0xd2, 0x7f, 0x2b, 0x2c)]
interface _PermissionSet(_PermissionSetVtbl) : IDispatch(IDispatchVtbl){
}}

RIDL!{#[uuid(0xbebb2505, 0x8b54, 0x3443, 0xae, 0xad, 0x14, 0x2a, 0x16, 0xdd, 0x9c, 0xc7)]
interface _AssemblyBuilder(_AssemblyBuilderVtbl): IUnknown(IUnknownVtbl){
    fn get_type_info_count(
        pc_t_info: *mut ULONG,
    ) -> HRESULT,
    fn get_type_info(
        i_t_info: ULONG, 
        lcid: ULONG, 
        pp_t_info: c_long, 
    ) -> HRESULT,
    fn get_ids_of_names(
        riid: *const GUID, 
        rgsz_names: c_long, 
        c_names: ULONG, 
        lcid: ULONG, 
        rg_disp_id: c_long,
    ) -> HRESULT, 
    fn invoke(
        disp_id_member: ULONG, 
        riid: *const GUID, 
        lcid: ULONG, 
        w_flags: c_short, 
        p_disp_params: c_long, 
        p_var_result: c_long, 
        p_excep_info: c_long, 
        pu_arg_err: c_long,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x05f696dc, 0x2b29, 0x3663, 0xad, 0x8b, 0xc4, 0x38, 0x9c, 0xf2, 0xa7, 0x13)]
interface _AppDomain(_AppDomainVtbl): IUnknown(IUnknownVtbl){
    fn get_type_info_count(
        pc_t_info: *mut ULONG,
    ) -> HRESULT,
    fn get_type_info(
        i_t_info: ULONG,
        lcid: ULONG, 
        pp_t_info: c_long,
    ) -> HRESULT,
    fn get_ids_of_names(
        riid: *const GUID, 
        rgsz_names: c_long, 
        c_names: ULONG, 
        lcid: ULONG, 
        rg_disp_id: c_long,
    ) -> HRESULT,
    fn invoke(
        disp_id_member: ULONG, 
        riid: *const GUID, 
        lcid: ULONG,
        w_flags: c_short, 
        p_disp_params: c_long, 
        p_var_result: c_long, 
        p_excep_info: c_long, 
        pu_arg_err: c_long,
    ) -> HRESULT,
    fn get_to_string(
        p_ret: *mut BSTR, 
    ) -> HRESULT,
    fn equals(
        other: VARIANT, 
        p_ret: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_hash_code(
        p_ret: *mut c_long, 
    ) -> HRESULT, 
    fn get_type(
        p_ret: *mut *mut _Type, 
    ) -> HRESULT, 
    fn initialize_lifetime_service(
        p_ret: *mut VARIANT, 
    ) -> HRESULT, 
    fn get_lifetime_service(
        p_ret: *mut VARIANT, 
    ) -> HRESULT,
    fn get_evidence(
        p_ret: *mut *mut _Evidence, 
    ) -> HRESULT, 
    fn add_domain_unload(
        val: *mut _EventHandler, 
    ) -> HRESULT, 
    fn remove_domain_unload(
        val: *mut _EventHandler, 
    ) -> HRESULT,
    fn add_assembly_load(
        val: *mut _AssemblyLoadEventHandler,
    ) -> HRESULT, 
    fn remove_assembly_load(
        val: *mut _AssemblyLoadEventHandler,
    ) -> HRESULT, 
    fn add_process_exit(
        val: *mut _EventHandler, 
    ) -> HRESULT, 
    fn remove_process_exit(
        val: *mut _EventHandler, 
    ) -> HRESULT, 
    fn add_type_resolve(
        val: *mut _ResolveEventHandler, 
    ) -> HRESULT, 
    fn remove_type_resolve(
        val: *mut _ResolveEventHandler, 
    ) -> HRESULT, 
    fn add_resource_resolve(
        val: *mut _ResolveEventHandler, 
    ) -> HRESULT, 
    fn remove_resource_resolve(
        val: *mut _ResolveEventHandler, 
    ) -> HRESULT, 
    fn add_assembly_resolve(
        val: *mut _ResolveEventHandler, 
    ) -> HRESULT, 
    fn remove_assembly_resolve(
        val: *mut _ResolveEventHandler, 
    ) -> HRESULT, 
    fn add_unhandled_exception(
        val: *mut _UnhandledExceptionEventHandler, 
    ) -> HRESULT, 
    fn remove_unhandled_exception(
        val: *mut _UnhandledExceptionEventHandler, 
    ) -> HRESULT, 
    fn define_dynamic_assembly(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_2(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        dir: BSTR,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_3(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        evidence: *const _Evidence,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_4(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        required_permissions: *const _PermissionSet, 
        optional_permissions: *const _PermissionSet, 
        refused_permissions: *const _PermissionSet,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_5(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        dir: BSTR,
        evidence: *const _Evidence,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_6(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        dir: BSTR, 
        required_permissions: *const _PermissionSet, 
        optional_permissions: *const _PermissionSet, 
        refused_permissions: *const _PermissionSet,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_7(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        evidence: *const _Evidence, 
        required_permissions: *const _PermissionSet, 
        optional_permissions: *const _PermissionSet, 
        refused_permissions: *const _PermissionSet,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_8(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        dir: BSTR, 
        evidence: *const _Evidence,
        required_permissions: *const _PermissionSet, 
        optional_permissions: *const _PermissionSet, 
        refused_permissions: *const _PermissionSet,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn define_dynamic_assembly_9(
        name: *const _AssemblyName, 
        access: AssemblyBuilderAccess, 
        dir: BSTR, 
        evidence: *const _Evidence, 
        required_permissions: *const _PermissionSet, 
        optional_permissions: *const _PermissionSet, 
        refused_permissions: *const _PermissionSet,
        is_synchronized: VARIANT_BOOL,
        p_ret: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn create_instance(
        assembly_name: BSTR, 
        type_name: BSTR, 
        p_ret: *mut *mut _ObjectHandle, 
    ) -> HRESULT, 
    fn create_instance_from(
        assembly_file: BSTR, 
        type_name: BSTR, 
        p_ret: *mut *mut _ObjectHandle, 
    ) -> HRESULT, 
    fn create_instance_2(
        assembly_name: BSTR, 
        type_name: BSTR, 
        activation_attributes: *const SAFEARRAY, 
        p_ret: *mut *mut _ObjectHandle, 
    ) -> HRESULT, 
    fn create_instance_from_2(
        assembly_file: BSTR, 
        type_name: BSTR, 
        activation_attributes: *const SAFEARRAY, 
        p_ret: *mut *mut _ObjectHandle, 
    ) -> HRESULT, 
    fn create_instance_3(
        assembly_name: BSTR, 
        type_name: BSTR, 
        ignore_case: VARIANT_BOOL, 
        binding_attr: BindingFlags, 
        binder: *const _Binder, 
        args: *const SAFEARRAY, 
        culture: *const _CultureInfo, 
        activation_attributes: *const SAFEARRAY, 
        security_attributes: *const _Evidence, 
        p_ret: *mut *mut _ObjectHandle, 
    ) -> HRESULT, 
    fn create_instance_from_3(
        assembly_file: BSTR, 
        type_name: BSTR, 
        ignore_case: VARIANT_BOOL, 
        binding_attr: BindingFlags, 
        binder: *const _Binder,
        args: *const SAFEARRAY, 
        culture: *const _CultureInfo, 
        activation_attributes: *const SAFEARRAY, 
        security_attributes: *const _Evidence, 
        p_ret: *mut *mut _ObjectHandle,
    ) -> HRESULT, 
    fn load(
        assembly_ref: *const _AssemblyName, 
        p_ret: *mut *mut _Assembly, 
    ) -> HRESULT, 
    fn load_2(
        assembly_string: BSTR, 
        p_ret: *mut *mut _Assembly, 
    ) -> HRESULT,
    fn load_3(
        raw_assembly: *const SAFEARRAY, 
        p_ret: *mut *mut _Assembly,
    ) -> HRESULT, 
    fn load_4(
        raw_assembly: *const SAFEARRAY, 
        raw_symbol_store: *const SAFEARRAY, 
        p_ret: *mut *mut _Assembly,
    ) -> HRESULT, 
    fn load_5(
        raw_assembly: *const SAFEARRAY, 
        raw_symbol_store: *const SAFEARRAY, 
        security_evidence: *const _Evidence,
        p_ret: *mut *mut _Assembly,
    ) -> HRESULT, 
    fn load_6(
        assembly_name: *const _AssemblyName, 
        assembly_security: *const _Evidence, 
        p_ret: *mut *mut _Assembly,
    ) -> HRESULT, 
    fn load_7(
        assembly_string: BSTR, 
        assembly_security: *const _Evidence, 
        p_ret: *mut *mut _Assembly,
    ) -> HRESULT, 
    fn execute_assembly(
        assembly_file: BSTR, 
        assembly_security: *const _Evidence, 
        p_ret: *mut c_long,
    ) -> HRESULT, 
    fn execute_assembly_2(
        assembly_file: BSTR, 
        p_ret: *mut c_long, 
    ) -> HRESULT, 
    fn execute_assembly_3(
        assembly_file: BSTR, 
        assembly_security: *const _Evidence, 
        args: *const SAFEARRAY, 
        p_ret: *mut c_long, 
    ) -> HRESULT, 
    fn get_friendly_name(
        p_ret: *mut BSTR, 
    ) -> HRESULT, 
    fn get_base_directory(
        p_ret: *mut BSTR, 
    ) -> HRESULT, 
    fn get_relative_search_path(
        p_ret: *mut BSTR, 
    ) -> HRESULT, 
    fn get_shadow_copy_files(
        p_ret: *mut VARIANT_BOOL, 
    ) -> HRESULT, 
    fn get_assemblies(
        p_ret: *mut *mut SAFEARRAY, 
    ) -> HRESULT, 
    fn append_private_path(
        path: BSTR, 
    ) -> HRESULT, 
    fn clear_private_path() -> HRESULT, 
    fn set_shadow_copy_path(
        s: BSTR,
    ) -> HRESULT, 
    fn clear_shadow_copy_path() -> HRESULT,
    fn set_cache_path(
        s: BSTR, 
    ) -> HRESULT,
    fn set_data(
        name: BSTR, 
        data: VARIANT,
    ) -> HRESULT,   
    fn get_data(
        name: BSTR, 
        p_ret: *mut VARIANT, 
    ) -> HRESULT, 
    fn set_app_domain_policy(
        domain_policy: *const _PolicyLevel,
    ) -> HRESULT, 
    fn set_thread_principal(
        principal: *const IPrincipal,
    ) -> HRESULT, 
    fn set_principal_policy(
        policy: PrincipalPolicy,
    ) -> HRESULT, 
    fn do_call_back(
        the_delegate: *const _CrossAppDomainDelegate,
    ) -> HRESULT, 
    fn get_dynamic_directory(
        p_ret: *mut BSTR, 
    ) -> HRESULT, 
}}

