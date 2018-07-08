///std

//3rd party 
use winapi::ctypes::{c_long, c_short};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

// self
mod enums;
mod core;

pub use self::enums::*;
pub use self::core::*;

RIDL!{#[uuid(0xaf93163f, 0xc2f4, 0x3fab, 0x9f, 0xf1, 0x72, 0x8a, 0x7a, 0xaa, 0xd1, 0xcb)]
interface _CrossAppDomainDelegate(_CrossAppDomainDelegateVtbl) : IDispatch(IDispatchVtbl){
}}

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
