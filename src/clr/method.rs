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
    MethodImplAttributes_CodeTypeMask = 3,
    MethodImplAttributes_IL = 0,
    MethodImplAttributes_Native = 1,
    MethodImplAttributes_OPTIL = 2,
    MethodImplAttributes_Runtime = -3,
    MethodImplAttributes_ManagedMask = 4,
    MethodImplAttributes_Unmanaged = -4,
    MethodImplAttributes_Managed = -1,
    MethodImplAttributes_ForwardRef = 16,
    MethodImplAttributes_PreserveSig = 128,
    MethodImplAttributes_InternalCall = 4096,
    MethodImplAttributes_Synchronized = 32,
    MethodImplAttributes_NoInlining = 8,
    MethodImplAttributes_NoOptimization = 64,
    MethodImplAttributes_MaxMethodImplVal = 65535
}

impl Interface for MethodImplAttributes {
    #[inline]
    fn uuidof() -> GUID {
        GUID{
            Data1: 0xbcab3a5d,
            Data2: 0xf2cd,
            Data3: 0x3c69,
            Data4: [0x84, 0x1d, 0xad, 0x00, 0x19, 0x69, 0xbf, 0x50],
        }
    }
}


pub struct RuntimeMethodHandle
{
    m_value: *mut IUnknown,
}

impl Interface for RuntimeMethodHandle {
    #[inline]
    fn uuidof() -> GUID {
        GUID{
            Data1: 0xf8fc5d7c,
            Data2: 0x8215,
            Data3: 0x3e65,
            Data4: [0xbe, 0xfb, 0x11, 0xe8, 0x17, 0x26, 0x06, 0xfe],
        }
    }
}

RIDL!{#[uuid(0xffcc1b5d, 0xecb8, 0x38dd, 0x9b, 0x01, 0x3d, 0xc8, 0xab, 0xc2, 0xaa, 0x5f)]
interface _MethodInfo(_MethodInfoVtbl): IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount(
        pcTInfo: *mut ULONG,
    ) -> HRESULT,
    fn GetTypeInfo(
        iTInfo: ULONG, 
        lcid: ULONG,
        ppTInfo: c_long,
    ) -> HRESULT,
    fn GetIDsOfNames(
        riid: *mut GUID,
        rgszNames: c_long, 
        cNames: ULONG, 
        lcid: ULONG, 
        rgDispId: c_long,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: ULONG, 
        riid: *mut GUID, 
        lcid: ULONG, 
        wFlags: c_short,
        pDispParams: c_long, 
        pVarResult: c_long, 
        pExcepInfo: c_long, 
        puArgErr: c_long,
    ) -> HRESULT,
    fn get_ToString(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn Equals(
        other: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes,
    ) -> HRESULT,
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY, 
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: *mut *mut MethodImplAttributes, 
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal: *mut RuntimeMethodHandle,
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
    fn GetTypeInfoCount(
        pcTInfo: *mut ULONG,
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