//std

//3rd party
use winapi::Interface;
use winapi::ctypes::{c_long, c_short, c_void};
use winapi::shared::guiddef::{GUID, REFIID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{LPCWSTR};

//self
use clr::field::{_FieldInfo, _PropertyInfo};
use clr::type_::_Type;
use clr::method::_MethodBase;

//body
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

pub enum StreamingContextStates
{
    StreamingContextStates_CrossProcess = 1,
    StreamingContextStates_CrossMachine = 2,
    StreamingContextStates_File = 4,
    StreamingContextStates_Persistence = 8,
    StreamingContextStates_Remoting = 16,
    StreamingContextStates_Other = 32,
    StreamingContextStates_Clone = 64,
    StreamingContextStates_CrossAppDomain = 128,
    StreamingContextStates_All = 255
}

impl Interface for StreamingContextStates {
    #[inline]
    fn uuidof() -> GUID {
        GUID{
            Data1: 0x78304e50,
            Data2: 0xa1e6,
            Data3: 0x3d84,
            Data4: [0xa7, 0x18, 0x49, 0x02, 0x06, 0x81, 0xe0, 0x2e],
        }
    }
}

pub struct StreamingContext {
    m_additionalContext: *mut IUnknown,
    m_state: StreamingContextStates
}

impl Interface for StreamingContext {
    #[inline]
    fn uuidof() -> GUID {
        GUID {
            Data1: 0x79179aa0,
            Data2: 0xe14c,
            Data3: 0x35ea,
            Data4: [0xa6, 0x66, 0x66, 0xbe, 0x96, 0x8a, 0xf6, 0x9f],
        }
    }
}
RIDL!{#[uuid(0x05532e88, 0xe0f2, 0x3263, 0x9b, 0x57, 0x80, 0x5a, 0xc6, 0xb6, 0xbb, 0x72)]
interface _ModuleResolveEventHandler(_ModuleResolveEventHandlerVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x152722c2, 0xf0b1, 0x3d19, 0xad, 0xa8, 0xf4, 0x0c, 0xa5, 0xca, 0xec, 0xb8)]
interface _CultureInfo(_CultureInfoVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x011a90c5, 0x4910, 0x3c29, 0xbb, 0xb7, 0x50, 0xd0, 0x5c, 0xcb, 0xaa, 0x4a)]
interface _Version(_VersionVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0xd002e9ba, 0xd9e3, 0x3749, 0xb1, 0xd3, 0xd5, 0x65, 0xa0, 0x8b, 0x13, 0xe7)]
interface _Module(_ModuleVtbl): IUnknown(IUnknownVtbl) {
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
}} 

#[repr(C)]
pub enum BindingFlags
{
    BindingFlags_Default = 0,
    BindingFlags_IgnoreCase = 1,
    BindingFlags_DeclaredOnly = 2,
    BindingFlags_Instance = 4,
    BindingFlags_Static = 8,
    BindingFlags_Public = 16,
    BindingFlags_NonPublic = 32,
    BindingFlags_FlattenHierarchy = 64,
    BindingFlags_InvokeMethod = 256,
    BindingFlags_CreateInstance = 512,
    BindingFlags_GetField = 1024,
    BindingFlags_SetField = 2048,
    BindingFlags_GetProperty = 4096,
    BindingFlags_SetProperty = 8192,
    BindingFlags_PutDispProperty = 16384,
    BindingFlags_PutRefDispProperty = 32768,
    BindingFlags_ExactBinding = 65536,
    BindingFlags_SuppressChangeType = 131072,
    BindingFlags_OptionalParamBinding = 262144,
    BindingFlags_IgnoreReturn = 16777216
}

impl Interface for BindingFlags {
    #[inline]
    fn uuidof() -> GUID {
        GUID {
            Data1: 0x3223e024,
            Data2: 0x5d70,
            Data3: 0x3236,
            Data4: [0xa9, 0x2a, 0x6b, 0x41, 0x14, 0xb2, 0x63, 0x2f],
        }
    }
}

RIDL!{#[uuid(0x3169ab11, 0x7109, 0x3808, 0x9a, 0x61, 0xef, 0x4b, 0xa0, 0x53, 0x4f, 0xd9)]
interface _Binder(_BinderVtbl): IDispatch(IDispatchVtbl){
    fn get_ToString(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,

    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT,
    
    fn GetHashCode(
        pRetVal: *mut c_long, 
    ) -> HRESULT,

    fn GetType(
        pRetVal: *mut *mut _Type, 
    ) -> HRESULT,

    fn BindToMethod(
        bindingAttr: BindingFlags,
        match_: *mut SAFEARRAY,
        args: *mut *mut SAFEARRAY,
        modifiers: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        names: *mut SAFEARRAY, 
        state: *mut VARIANT,
        pRetVal: *mut *mut _MethodBase, 
    ) -> HRESULT,

    fn BindToField(
        bindingAttr: BindingFlags,
        match_: *const SAFEARRAY,
        val: VARIANT, 
        culture: *const _CultureInfo, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    
    fn SelectMethod(
        bindingAttr: BindingFlags,
        match_: *const SAFEARRAY, 
        types: *const SAFEARRAY, 
        modifiers: *const SAFEARRAY, 
        pRetVal: *mut *mut _MethodBase, 
    ) -> HRESULT,
    
    fn SelectProperty(
        bindingAttr: BindingFlags,
        match_: *const SAFEARRAY,
        returnType: *const _Type,
        index: *const SAFEARRAY, 
        modifiers: *const SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    
    fn ChangeType(
        val: VARIANT,
        Type: *const _Type,
        culture: *const _CultureInfo,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    
    fn ReorderArgumentArray(
        args: *mut *mut SAFEARRAY,
        state: VARIANT, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9065597E, 0xD1A1, 0x4fb2, 0xB6, 0xBA, 0x7E, 0x1F, 0xCE, 0x23, 0x0F, 0x61)]
interface ICLRControl(ICLRControlVtbl): IUnknown(IUnknownVtbl) {
	fn GetCLRManager(
		riid: REFIID, 
		ppObject: *mut *mut c_void,
	) -> HRESULT, 
	fn SetAppDomainManagerType(
		pwzAppDomainManagerAssembly: LPCWSTR, 
		pwzAppDomainManagerType: LPCWSTR,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x05f696dc, 0x2b29, 0x3663, 0xad, 0x8b, 0xc4, 0x38, 0x9c, 0xf2, 0xa7, 0x13)]
interface _AppDomain(_AppDomainVtbl): IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount(
        pcTInfo: *mut ULONG,
    ) -> HRESULT,
}}
/*    virtual HRESULT __stdcall GetTypeInfo (
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
      virtual HRESULT __stdcall InitializeLifetimeService (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetLifetimeService (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Evidence (
        /*[out,retval]*/ struct _Evidence * * pRetVal ) = 0;
      virtual HRESULT __stdcall add_DomainUnload (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_DomainUnload (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall add_AssemblyLoad (
        /*[in]*/ struct _AssemblyLoadEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_AssemblyLoad (
        /*[in]*/ struct _AssemblyLoadEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_ProcessExit (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_ProcessExit (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall add_TypeResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_TypeResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_ResourceResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_ResourceResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_AssemblyResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_AssemblyResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_UnhandledException (
        /*[in]*/ struct _UnhandledExceptionEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_UnhandledException (
        /*[in]*/ struct _UnhandledExceptionEventHandler * val ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_2 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_3 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ struct _Evidence * Evidence,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_4 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_5 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _Evidence * Evidence,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_6 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_7 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ struct _Evidence * Evidence,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_8 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _Evidence * Evidence,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_9 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _Evidence * Evidence,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[in]*/ VARIANT_BOOL IsSynchronized,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance (
        /*[in]*/ BSTR AssemblyName,
        /*[in]*/ BSTR typeName,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstanceFrom (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ BSTR typeName,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance_2 (
        /*[in]*/ BSTR AssemblyName,
        /*[in]*/ BSTR typeName,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstanceFrom_2 (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ BSTR typeName,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance_3 (
        /*[in]*/ BSTR AssemblyName,
        /*[in]*/ BSTR typeName,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[in]*/ struct _Evidence * securityAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstanceFrom_3 (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ BSTR typeName,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[in]*/ struct _Evidence * securityAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load (
        /*[in]*/ struct _AssemblyName * assemblyRef,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_2 (
        /*[in]*/ BSTR assemblyString,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_3 (
        /*[in]*/ SAFEARRAY * rawAssembly,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_4 (
        /*[in]*/ SAFEARRAY * rawAssembly,
        /*[in]*/ SAFEARRAY * rawSymbolStore,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_5 (
        /*[in]*/ SAFEARRAY * rawAssembly,
        /*[in]*/ SAFEARRAY * rawSymbolStore,
        /*[in]*/ struct _Evidence * securityEvidence,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_6 (
        /*[in]*/ struct _AssemblyName * assemblyRef,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_7 (
        /*[in]*/ BSTR assemblyString,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall ExecuteAssembly (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall ExecuteAssembly_2 (
        /*[in]*/ BSTR assemblyFile,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall ExecuteAssembly_3 (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[in]*/ SAFEARRAY * args,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FriendlyName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_BaseDirectory (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_RelativeSearchPath (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ShadowCopyFiles (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetAssemblies (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall AppendPrivatePath (
        /*[in]*/ BSTR Path ) = 0;
      virtual HRESULT __stdcall ClearPrivatePath ( ) = 0;
      virtual HRESULT __stdcall SetShadowCopyPath (
        /*[in]*/ BSTR s ) = 0;
      virtual HRESULT __stdcall ClearShadowCopyPath ( ) = 0;
      virtual HRESULT __stdcall SetCachePath (
        /*[in]*/ BSTR s ) = 0;
      virtual HRESULT __stdcall SetData (
        /*[in]*/ BSTR name,
        /*[in]*/ VARIANT data ) = 0;
      virtual HRESULT __stdcall GetData (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall SetAppDomainPolicy (
        /*[in]*/ struct _PolicyLevel * domainPolicy ) = 0;
      virtual HRESULT __stdcall SetThreadPrincipal (
        /*[in]*/ struct IPrincipal * principal ) = 0;
      virtual HRESULT __stdcall SetPrincipalPolicy (
        /*[in]*/ enum PrincipalPolicy policy ) = 0;
      virtual HRESULT __stdcall DoCallBack (
        /*[in]*/ struct _CrossAppDomainDelegate * theDelegate ) = 0;
      virtual HRESULT __stdcall get_DynamicDirectory (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;*/