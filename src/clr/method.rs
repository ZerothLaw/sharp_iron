//method.rs
//std

//3rd party
use winapi::Interface;

use winapi::ctypes::{c_long, c_short};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{SAFEARRAY, VARIANT};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
//self
use clr::type_::_Type;
use clr::field::MemberTypes;

#[repr(C)]
pub enum MethodImplAttributes {
    CodeTypeMask = 3,
    IL = 0,
    Native = 1,
    OPTIL = 2,
    Runtime = -3,
    ManagedMask = 4,
    Unmanaged = -4,
    Managed = -1,
    ForwardRef = 16,
    PreserveSig = 128,
    InternalCall = 4096,
    Synchronized = 32,
    NoInlining = 8,
    NoOptimization = 64,
    MaxMethodImplVal = 65535
}
add_uuid!(MethodImplAttributes, 0xbcab3a5d, 0xf2cd, 0x3c69,0x84, 0x1d, 0xad, 0x00, 0x19, 0x69, 0xbf, 0x50);

#[repr(C)]
pub struct RuntimeMethodHandle
{
    m_value: *mut IUnknown,
}
add_uuid!(RuntimeMethodHandle, 0xf8fc5d7c, 0x8215, 0x3e65, 0xbe, 0xfb, 0x11, 0xe8, 0x17, 0x26, 0x06, 0xf);

RIDL!{#[uuid(0xffcc1b5d, 0xecb8, 0x38dd, 0x9b, 0x01, 0x3d, 0xc8, 0xab, 0xc2, 0xaa, 0x5f)]
interface _MethodInfo(_MethodInfoVtbl): IUnknown(IUnknownVtbl){
    fn get_type_info_count(
        pc_t_info: *mut ULONG,
    ) -> HRESULT,
    fn get_type_info(
        i_t_info: ULONG, 
        lcid: ULONG,
        pp_t_info: c_long,
    ) -> HRESULT,
    fn get_ids_of_names(
        riid: *mut GUID,
        rgsz_names: c_long, 
        c_names: ULONG, 
        lcid: ULONG, 
        rg_disp_id: c_long,
    ) -> HRESULT,
    fn invoke(
        disp_id_member: ULONG, 
        riid: *mut GUID, 
        lcid: ULONG, 
        w_flags: c_short,
        p_disp_params: c_long, 
        p_var_result: c_long, 
        p_excep_info: c_long, 
        pu_arg_err: c_long,
    ) -> HRESULT,
    fn to_string(
        p_ret_val: *mut BSTR, 
    ) -> HRESULT, 
    fn equals(
        other: VARIANT, 
        p_ret_val: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_hash_code(
        p_ret_val: *mut c_long,
    ) -> HRESULT,
    fn get_type(
        p_ret_val: *mut *mut _Type,
    ) -> HRESULT,
    fn get_member_type(
        p_ret_val: *mut MemberTypes,
    ) -> HRESULT,
    fn get_name(
        p_ret_val: *mut BSTR,
    ) -> HRESULT,
    fn get_declaring_type(
        p_ret_val: *mut *mut _Type,
    ) -> HRESULT,
    fn get_reflected_type(
        p_ret_val: *mut *mut _Type,
    ) -> HRESULT,
    fn get_custom_attributes(
        attribute_type: *mut _Type,
        inherit: VARIANT_BOOL,
        p_ret_val: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_custom_attributes_2(
        inherit: VARIANT_BOOL, 
        p_ret_val: *mut *mut SAFEARRAY, 
    ) -> HRESULT,
    fn is_defined(
        attribute_type: *mut _Type,
        inherit: VARIANT_BOOL, 
        p_ret_val: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_parameters(
        p_ret_val: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_method_implementation_flags(
        p_ret_val: *mut *mut MethodImplAttributes, 
    ) -> HRESULT,
    fn get_method_handle(
        p_ret_val: *mut RuntimeMethodHandle,
    ) -> HRESULT,
    /*virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum MethodAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CallingConvention (
        /*[out,retval]*/ enum CallingConventions * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyAndAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyOrAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsStatic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFinal (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsVirtual (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsHideBySig (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsConstructor (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_3 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_returnType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReturnTypeCustomAttributes (
        /*[out,retval]*/ struct ICustomAttributeProvider * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetBaseDefinition (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;*/
}}


RIDL!{#[uuid(0x6240837a, 0x707f, 0x3181, 0x8e, 0x98, 0xa3, 0x6a, 0xe0, 0x86, 0x76, 0x6b)]
interface _MethodBase(_MethodBaseVtbl): IUnknown(IUnknownVtbl)
{
    fn get_type_info_count(
        pc_t_info: *mut ULONG,
    ) -> HRESULT,
/*virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetParameters (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethodImplementationFlags (
        /*[out,retval]*/ enum MethodImplAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodHandle (
        /*[out,retval]*/ struct RuntimeMethodHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum MethodAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CallingConvention (
        /*[out,retval]*/ enum CallingConventions * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyAndAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyOrAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsStatic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFinal (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsVirtual (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsHideBySig (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsConstructor (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_3 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;*/
}}