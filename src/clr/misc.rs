
#![allow(dead_code)]
#![allow(non_snake_case)]
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

#[repr(C)]
pub enum StreamingContextStates
{
    CrossProcess = 1,
    CrossMachine = 2,
    File = 4,
    Persistence = 8,
    Remoting = 16,
    Other = 32,
    Clone_ = 64,
    CrossAppDomain = 128,
    All = 255
}
add_uuid!(StreamingContextStates, 0x78304e50, 0xa1e6, 0x3d84, 0xa7, 0x18, 0x49, 0x02, 0x06, 0x81, 0xe0, 0x2e);

#[repr(C)]
pub struct StreamingContext {
    m_additional_context: *mut IUnknown,
    m_state: StreamingContextStates
}
add_uuid!(StreamingContext, 0x79179aa0, 0xe14c, 0x35ea, 0xa6, 0x66, 0x66, 0xbe, 0x96, 0x8a, 0xf6, 0x9f);

RIDL!{#[uuid(0x05532e88, 0xe0f2, 0x3263, 0x9b, 0x57, 0x80, 0x5a, 0xc6, 0xb6, 0xbb, 0x72)]
interface _ModuleResolveEventHandler(_ModuleResolveEventHandlerVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x152722c2, 0xf0b1, 0x3d19, 0xad, 0xa8, 0xf4, 0x0c, 0xa5, 0xca, 0xec, 0xb8)]
interface _CultureInfo(_CultureInfoVtbl): IDispatch(IDispatchVtbl){}}

RIDL!{#[uuid(0x011a90c5, 0x4910, 0x3c29, 0xbb, 0xb7, 0x50, 0xd0, 0x5c, 0xcb, 0xaa, 0x4a)]
interface _Version(_VersionVtbl): IDispatch(IDispatchVtbl){}}

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

RIDL!{#[uuid(0xd002e9ba, 0xd9e3, 0x3749, 0xb1, 0xd3, 0xd5, 0x65, 0xa0, 0x8b, 0x13, 0xe7)]
interface _Module(_ModuleVtbl): IUnknown(IUnknownVtbl) {
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
}} 

#[repr(C)]
pub enum BindingFlags
{
    Default_ = 0,
    IgnoreCase = 1,
    DeclaredOnly = 2,
    Instance = 4,
    Static = 8,
    Public = 16,
    NonPublic = 32,
    FlattenHierarchy = 64,
    InvokeMethod = 256,
    CreateInstance = 512,
    GetField = 1024,
    SetField = 2048,
    GetProperty = 4096,
    SetProperty = 8192,
    PutDispProperty = 16384,
    PutRefDispProperty = 32768,
    ExactBinding = 65536,
    SuppressChangeType = 131072,
    OptionalParamBinding = 262144,
    IgnoreReturn = 16777216
}
add_uuid!(BindingFlags, 0x3223e024, 0x5d70, 0x3236, 0xa9, 0x2a, 0x6b, 0x41, 0x14, 0xb2, 0x63, 0x2f);

pub enum FieldAttributes
{
    FieldAccessMask = 7,
    PrivateScope = 0,
    Private = 1,
    FamANDAssem = 2,
    Assembly = 3,
    Family = 4,
    FamORAssem = 5,
    Public = 6,
    Static = 16,
    InitOnly = 32,
    Literal = 64,
    NotSerialized = 128,
    SpecialName = 512,
    PinvokeImpl = 8192,
    ReservedMask = 38144,
    RTSpecialName = 1024,
    HasFieldMarshal = 4096,
    HasDefault = 32768,
    HasFieldRVA = 256
}
add_uuid!(FieldAttributes, 0xc8679e0a, 0x1c67, 0x3a20, 0x86, 0x45, 0x0d, 0x93, 0x0f, 0x52, 0x90, 0x31);

RIDL!{#[uuid(0x3169ab11, 0x7109, 0x3808, 0x9a, 0x61, 0xef, 0x4b, 0xa0, 0x53, 0x4f, 0xd9)]
interface _Binder(_BinderVtbl): IDispatch(IDispatchVtbl){
    fn to_string(
        p_ret_val: *mut BSTR, 
    ) -> HRESULT,

    fn equals(
        obj: VARIANT,
        p_ret_val: *mut VARIANT_BOOL, 
    ) -> HRESULT,
    
    fn get_hash_code(
        p_ret_val: *mut c_long, 
    ) -> HRESULT,

    fn get_type(
        p_ret_val: *mut *mut _Type, 
    ) -> HRESULT,

    fn bind_to_method(
        binding_attr: BindingFlags,
        match_: *mut SAFEARRAY,
        args: *mut *mut SAFEARRAY,
        modifiers: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        names: *mut SAFEARRAY, 
        state: *mut VARIANT,
        p_ret_val: *mut *mut _MethodBase, 
    ) -> HRESULT,

    fn bind_to_field(
        binding_attr: BindingFlags,
        match_: *const SAFEARRAY,
        val: VARIANT, 
        culture: *const _CultureInfo, 
        p_ret_val: *mut *mut _FieldInfo,
    ) -> HRESULT,
    
    fn select_method(
        binding_attr: BindingFlags,
        match_: *const SAFEARRAY, 
        types: *const SAFEARRAY, 
        modifiers: *const SAFEARRAY, 
        p_ret_val: *mut *mut _MethodBase, 
    ) -> HRESULT,
    
    fn select_property(
        binding_attr: BindingFlags,
        match_: *const SAFEARRAY,
        return_type: *const _Type,
        index: *const SAFEARRAY, 
        modifiers: *const SAFEARRAY, 
        p_ret_val: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    
    fn change_type(
        val: VARIANT,
        type_: *const _Type,
        culture: *const _CultureInfo,
        p_ret_val: *mut VARIANT, 
    ) -> HRESULT,
    
    fn reorder_argument_array(
        args: *mut *mut SAFEARRAY,
        state: VARIANT, 
    ) -> HRESULT,
}}

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

pub enum CallingConventions
{
    Standard = 1,
    VarArgs = 2,
    Any = 3,
    HasThis = 32,
    ExplicitThis = 64
}
add_uuid!(CallingConventions, 0xfd67ebe2, 0x30de, 0x3fbe, 0x89, 0x6b, 0x81, 0xda, 0x2e, 0x45, 0x51, 0x37);

pub enum TypeAttributes
{
    VisibilityMask = 7,
    NotPublic = 0,
    Public = 1,
    NestedPublic = 2,
    NestedPrivate = 3,
    NestedFamily = 4,
    NestedAssembly = 5,
    NestedFamANDAssem = 6,
    NestedFamORAssem = -7,
    LayoutMask = 24,
    AutoLayout,
    SequentialLayout = 8,
    ExplicitLayout = 16,
    ClassSemanticsMask = 32,
    Class,
    Interface = -32,
    Abstract = 128,
    Sealed = 256,
    SpecialName = 1024,
    Import = 4096,
    Serializable = 8192,
    StringFormatMask = 196608,
    AnsiClass,
    UnicodeClass = 65536,
    AutoClass = 131072,
    CustomFormatClass = -196608,
    CustomFormatMask = 12582912,
    BeforeFieldInit = 1048576,
    ReservedMask = 264192,
    RTSpecialName = 2048,
    HasSecurity = 262144
}
add_uuid!(TypeAttributes, 0x28ee6224, 0xfd72, 0x3bdf, 0xb2, 0x48, 0xba, 0x91, 0x02, 0xfc, 0xeb, 0x14);


RIDL!{#[uuid(0xe9a19478, 0x9646, 0x3679, 0x9b, 0x10, 0x84, 0x11, 0xae, 0x1f, 0xd5, 0x7d)]
interface _ConstructorInfo(_ConstructorInfoVtbl): IUnknown(IUnknownVtbl) {
    //
    // Raw methods provided by interface
    //

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
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_4 (
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_5 (
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;*/
}}

RIDL!{#[uuid(0x9de59c64, 0xd889, 0x35a1, 0xb8, 0x97, 0x58, 0x7d, 0x74, 0x46, 0x9e, 0x5b)]
interface _EventInfo(_EventInfoVtbl) : IUnknown(IUnknownVtbl)
{
    //
    // Raw methods provided by interface
    //
    /*
      virtual HRESULT __stdcall GetTypeInfoCount (
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
      virtual HRESULT __stdcall GetAddMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRemoveMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRaiseMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum EventAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall GetAddMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRemoveMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRaiseMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall AddEventHandler (
        /*[in]*/ VARIANT Target,
        /*[in]*/ struct _Delegate * handler ) = 0;
      virtual HRESULT __stdcall RemoveEventHandler (
        /*[in]*/ VARIANT Target,
        /*[in]*/ struct _Delegate * handler ) = 0;
      virtual HRESULT __stdcall get_EventHandlerType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsMulticast (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;*/
}}