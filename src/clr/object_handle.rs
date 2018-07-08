//object handle that wraps objects returned from app domains
//std

//3rd party
use winapi::ctypes::{c_long};

use winapi::shared::winerror::{HRESULT};

use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

//self
use clr::misc::_ObjRef;
use clr::type_::_Type;

//body

pub struct ObjectHandle {
    ptr: *mut _ObjectHandle,
}

RIDL!{#[uuid(0xea675b47, 0x64e0, 0x3b5f, 0x9b, 0xe7, 0xf7, 0xdc, 0x29, 0x90, 0x73, 0x0d)]
interface _ObjectHandle(_ObjectHandleVtbl) : IDispatch(IDispatchVtbl)
{
    fn get_to_string (
        p_ret: *mut BSTR, 
    ) ->HRESULT,
    fn equals(
        obj: VARIANT, 
        p_ret: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_hash_code(
        p_ret: *mut c_long, 
    ) -> HRESULT, 
    fn get_type(
        p_ret: *mut *mut _Type,
    ) -> HRESULT, 
    fn get_lifetime_service(
        p_ret: *mut VARIANT, 
    ) -> HRESULT, 
    fn initialize_lifetime_service(
        p_ret: *mut VARIANT, 
    ) -> HRESULT, 
    fn create_obj_ref(
        requested_type: *const _Type, 
        p_ret: *mut *mut _ObjRef,
    ) -> HRESULT, 
    fn unwrap(
        p_ret: *mut VARIANT, 
    ) -> HRESULT,
}}