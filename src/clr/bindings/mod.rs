///std

//3rd party 
use winapi::Interface;

use winapi::ctypes::{c_long, c_short, c_void};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};

use winapi::shared::guiddef::{REFIID};

use winapi::shared::minwindef::{DWORD};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

use winapi::um::oaidl::{SAFEARRAY};
use winapi::um::winnt::{LPCWSTR};

// self
mod enums;
mod core;
mod custom;

pub use self::enums::*;
pub use self::core::*;
pub use self::custom::*;

RIDL!{#[uuid(0xaf93163f, 0xc2f4, 0x3fab, 0x9f, 0xf1, 0x72, 0x8a, 0x7a, 0xaa, 0xd1, 0xcb)]
interface _CrossAppDomainDelegate(_CrossAppDomainDelegateVtbl) : IDispatch(IDispatchVtbl){
}}

RIDL!{#[uuid(0xc2af4970, 0x4fb6, 0x319c, 0xa8, 0xaa, 0x06, 0x14, 0xd2, 0x7f, 0x2b, 0x2c)]
interface _PermissionSet(_PermissionSetVtbl) : IDispatch(IDispatchVtbl){
}}

#[repr(C)]
pub struct RuntimeMethodHandle
{
    m_value: *mut IUnknown,
}
add_uuid!(RuntimeMethodHandle, 0xf8fc5d7c, 0x8215, 0x3e65, 0xbe, 0xfb, 0x11, 0xe8, 0x17, 0x26, 0x06, 0xf);

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

RIDL!{#[uuid(0x2752364a, 0x924f, 0x3603, 0x8f, 0x6f, 0x65, 0x86, 0xdf, 0x98, 0xb2, 0x92)]
interface _Stream(_StreamVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x74265195, 0x4a46, 0x3d6f, 0xa9, 0xdd, 0x69, 0xc3, 0x67, 0xea, 0x39, 0xc8)]
interface _FileStream(_FileStreamVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x3188878c, 0xdeb3, 0x3558, 0x80, 0xe8, 0x84, 0xe9, 0xed, 0x95, 0xf9, 0x2c)]
interface _ManifestResourceInfo(_ManifestResourceInfoVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0xa505edbc, 0x380e, 0x3b23, 0x9e, 0x1a, 0x09, 0x74, 0xd4, 0xef, 0x02, 0xef)]
interface _Evidence(_EvidenceVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0xb58d62cf, 0xb03a, 0x3a14, 0xb0, 0xb6, 0xb1, 0xe5, 0xad, 0x4e, 0x4a, 0xd5)]
interface _SerializationInfo(_SerializationInfoVtbl): IDispatch(IDispatchVtbl){}}


RIDL!{#[uuid(0x05532e88, 0xe0f2, 0x3263, 0x9b, 0x57, 0x80, 0x5a, 0xc6, 0xb6, 0xbb, 0x72)]
interface _ModuleResolveEventHandler(_ModuleResolveEventHandlerVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x152722c2, 0xf0b1, 0x3d19, 0xad, 0xa8, 0xf4, 0x0c, 0xa5, 0xca, 0xec, 0xb8)]
interface _CultureInfo(_CultureInfoVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x011a90c5, 0x4910, 0x3c29, 0xbb, 0xb7, 0x50, 0xd0, 0x5c, 0xcb, 0xaa, 0x4a)]
interface _Version(_VersionVtbl): IDispatch(IDispatchVtbl){}}

#[repr(C)]
pub struct StreamingContext {
    m_additional_context: *mut IUnknown,
    m_state: StreamingContextStates
}
add_uuid!(StreamingContext, 0x79179aa0, 0xe14c, 0x35ea, 0xa6, 0x66, 0x66, 0xbe, 0x96, 0x8a, 0xf6, 0x9f);

#[repr(C)]
pub struct RuntimeTypeHandle {
    m_type: *mut IUnknown
}
add_uuid!(RuntimeTypeHandle, 0x78c18a10, 0xc00e, 0x3c09, 0xb0, 0x00, 0x41, 0x1c, 0x38, 0x90, 0x0c, 0x2c);

#[repr(C)]
pub struct RuntimeFieldHandle {
    m_type: *mut IUnknown
}
add_uuid!(RuntimeFieldHandle, 0x27b33bd9, 0xe6f7, 0x3148, 0x91, 0x1d, 0xf6, 0x73, 0x40, 0xa5, 0x35, 0x3f);

RIDL!{#[uuid(0x9065597E, 0xD1A1, 0x4fb2, 0xB6, 0xBA, 0x7E, 0x1F, 0xCE, 0x23, 0x0F, 0x61)]
interface ICLRControl(ICLRControlVtbl): IUnknown(IUnknownVtbl) {
	fn clr_manager(
		riid: REFIID, 
		pp_object: *mut *mut c_void,
	) -> HRESULT, 
	fn set_app_domain_manager_type(
		pwz_app_domain_manager_assembly: LPCWSTR, 
		pwz_app_domain_manager_type: LPCWSTR,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x7cefc46e, 0x16e0, 0x3e65, 0x9c, 0x38, 0x55, 0xb4, 0x34, 0x2b, 0xa7, 0xf0)]
interface _EventHandler(_EventHandlerVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0xdeece11f, 0xa893, 0x3e35, 0xa4, 0xc3, 0xda, 0xb7, 0xfa, 0x09, 0x11, 0xeb)]
interface _AssemblyLoadEventHandler(_AssemblyLoadEventHandlerVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0x8e54a9cc, 0x7aa4, 0x34ca, 0x98, 0x5b, 0xbd, 0x7d, 0x75, 0x27, 0xb1, 0x10)]
interface _ResolveEventHandler(_ResolveEventHandlerVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0x84199e64, 0x439c, 0x3011, 0xb2, 0x49, 0x3c, 0x90, 0x65, 0x73, 0x5a, 0xdb)]
interface _UnhandledExceptionEventHandler(_UnhandledExceptionEventHandlerVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0x1dd3cf3d, 0xdf8e, 0x32ff, 0x91, 0xec, 0xe1, 0x9a, 0xa1, 0x0b, 0x63, 0xfb)]
interface _ObjRef(_ObjRefVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0x44494e35, 0xc370, 0x3014, 0xbc, 0x78, 0x0f, 0x2e, 0xcb, 0xf8, 0x3f, 0x53)]
interface _PolicyLevel(_PolicyLevelVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0x4283ca6c, 0xd291, 0x3481, 0x83, 0xc9, 0x95, 0x54, 0x48, 0x1f, 0xe8, 0x88)]
interface IPrincipal(IPrincipalVtbl) : IDispatch(IDispatchVtbl){
    fn identity(
        p_ret_val: *mut *mut IIdentity, 
    ) -> HRESULT, 
    fn is_in_role(
        role: BSTR, 
        p_ret_val: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0xf4205a87, 0x4d46, 0x303d, 0xb1, 0xd9, 0x5a, 0x99, 0xf7, 0xc9, 0x0d, 0x30)]
interface IIdentity(IIdentityVtbl) : IDispatch(IDispatchVtbl)
{
    fn name (
        p_ret_val: *mut BSTR,  
    ) -> HRESULT,
    fn authentication_type(
        p_ret_val: *mut BSTR, 
    ) -> HRESULT,
    fn is_authenticated(
        p_ret_val: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe1817846, 0x3745, 0x3c97, 0xb4, 0xa6, 0xee, 0x20, 0xa1, 0x64, 0x1b, 0x29)]
interface _TypeFilter(_TypeFilterVtbl) : IDispatch(IDispatchVtbl)
{}}

RIDL!{#[uuid(0xfae5d9b7, 0x40c1, 0x3de1, 0xbe, 0x06, 0xa9, 0x1c, 0x9d, 0xa1, 0xba, 0x9f)]
interface _MemberFilter(_MemberFilterVtbl) : IDispatch(IDispatchVtbl)
{}}

#[repr(C)]
pub struct InterfaceMapping
{
    target_type: *mut _Type,
    interface_type: *mut _Type,
    target_methods: *mut SAFEARRAY,
    interface_methods: *mut SAFEARRAY,
}
add_uuid!(InterfaceMapping, 0x5f7a2664, 0x4778, 0x3d72, 0xa7, 0x8f, 0xd3, 0x8b, 0x6b, 0x00, 0x18, 0x0d);

