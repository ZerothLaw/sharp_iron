//std

//3rd party
use winapi::ctypes::{c_int, c_long, c_short, c_void};

use winapi::um::winnt::{LPCWSTR};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{DWORD};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

//self

use clr::bindings::_AssemblyBuilder;
use clr::bindings::_AssemblyLoadEventHandler;
use clr::bindings::_CrossAppDomainDelegate;
use clr::bindings::_CultureInfo;
use clr::bindings::_EventHandler;
use clr::bindings::_Evidence;
use clr::bindings::_FileStream;
use clr::bindings::_ManifestResourceInfo;
use clr::bindings::_MemberFilter;
use clr::bindings::_ModuleResolveEventHandler;
use clr::bindings::_ObjectHandle;
use clr::bindings::_PolicyLevel;
use clr::bindings::_PermissionSet;
use clr::bindings::_ResolveEventHandler;
use clr::bindings::_SerializationInfo;
use clr::bindings::_Stream;
use clr::bindings::_TypeFilter;
use clr::bindings::_Version;
use clr::bindings::_UnhandledExceptionEventHandler;


use clr::bindings::AssemblyBuilderAccess;
use clr::bindings::ICLRControl;
use clr::bindings::IHostControl;
use clr::bindings::InterfaceMapping;
use clr::bindings::IPrincipal;
use clr::bindings::MemberTypes;
use clr::bindings::PrincipalPolicy;
use clr::bindings::PropertyAttributes;
use clr::bindings::RuntimeFieldHandle;
use clr::bindings::RuntimeMethodHandle;
use clr::bindings::RuntimeTypeHandle;
use clr::bindings::StreamingContext;

use clr::bindings::BindingFlags;
use clr::bindings::CallingConventions;
use clr::bindings::FieldAttributes;
use clr::bindings::MethodImplAttributes;
use clr::bindings::TypeAttributes;


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


RIDL!{#[uuid(0xb42b6aac, 0x317e, 0x34d5, 0x9f, 0xa9, 0x09, 0x3b, 0xb4, 0x16, 0x0c, 0x50)]
interface _AssemblyName(_AssemblyNameVtbl): IUnknown(IUnknownVtbl) {
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


RIDL!{#[uuid(0x17156360, 0x2f1a, 0x384a, 0xbc, 0x52, 0xfd, 0xe9, 0x3c, 0x21, 0x5c, 0x5b)]
interface _Assembly(_AssemblyVtbl): IDispatch(IDispatchVtbl){
	fn to_string(
		p_ret_val: *mut BSTR,
	) -> HRESULT,
	fn equals(
		other: VARIANT,
		p_ret_val: *mut VARIANT_BOOL,
	) -> HRESULT,
	fn hash_code(
		p_ret_val: *mut c_long,
	) -> HRESULT,
	fn get_type(
		p_ret_val: *mut *mut _Type,
	) -> HRESULT,
	fn code_base(
		p_ret_val: *mut BSTR,
	) -> HRESULT,
	fn escaped_code_base(
		p_ret_val: *mut BSTR,
	) -> HRESULT,
	fn get_name(
		p_ret_val: *mut *mut _AssemblyName,
	) -> HRESULT, 
	fn get_name_2(
		copied_name: VARIANT_BOOL, 
		p_ret_val: *mut *mut _AssemblyName,
	) -> HRESULT, 
	fn get_full_name(
		p_ret_val: *mut BSTR, 
	) -> HRESULT, 
	fn get_entry_point(
		p_ret_val: *mut *mut _MethodInfo, 
	) -> HRESULT,
	fn get_type_2(
		name: BSTR, 
		p_ret_val: *mut *mut _Type, 
	) -> HRESULT, 
	fn get_type_3(
		name: BSTR, 
		throw_on_error: VARIANT_BOOL, 
		p_ret_val: *mut *mut _Type,
	) -> HRESULT,

	fn get_exported_types(
		p_ret_val: *mut *mut SAFEARRAY, 
	) -> HRESULT,
	
	fn get_types(
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn get_manifest_resource_stream(
		type_: *mut _Type, 
		name: BSTR, 
		p_ret_val: *mut *mut _Stream,
	) -> HRESULT, 

	fn get_manifest_resource_stream_2(
		name: BSTR, 
		p_ret_val: *mut *mut _Stream,
	) -> HRESULT,

	fn get_file(
		name: BSTR, 
		p_ret_val: *mut *mut _FileStream,
	) -> HRESULT,

	fn get_files(
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn get_files_2(
		get_resource_modules: VARIANT_BOOL, 
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn get_manifest_resource_names(
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn get_manifest_resource_info(
		resource_name: BSTR, 
		p_ret_val: *mut *mut _ManifestResourceInfo,
	) -> HRESULT,

	fn get_location(
		p_ret_val: *mut BSTR, 
	) -> HRESULT, 

	fn get_evidence(
		p_ret_val: *mut *mut _Evidence,
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

	fn get_object_data(
		info: *mut _SerializationInfo, 
		context: StreamingContext,
	) -> HRESULT,

	fn add_module_resolve(
		val: *mut _ModuleResolveEventHandler, 
	) -> HRESULT, 

	fn remove_module_resolve(
		val: *mut _ModuleResolveEventHandler,
	) -> HRESULT, 

	fn get_type_4(
		name: BSTR, 
		throw_on_error: VARIANT_BOOL, 
		ignore_case: VARIANT_BOOL, 
		p_ret_val: *mut *mut _Type,
	) -> HRESULT, 

	fn get_satellite_assembly(
		culture: *mut _CultureInfo, 
		p_ret_val: *mut *mut _Assembly, 
	) -> HRESULT, 

	fn get_satellite_assembly_2(
		culture: *mut _CultureInfo, 
		Version: *mut _Version, 
		p_ret_val: *mut *mut _Assembly,
	) -> HRESULT,

	fn load_module(
		module_name: BSTR, 
		raw_module: *mut SAFEARRAY, 
		p_ret_val: *mut *mut _Module, 
	) -> HRESULT, 

	fn load_module_2(
		module_name: BSTR, 
		raw_module: *mut SAFEARRAY, 
		raw_symbol_store: *mut SAFEARRAY, 
		p_ret_val: *mut *mut _Module,
	) -> HRESULT,

	fn create_instance(
		type_name: BSTR, 
		p_ret_val: *mut VARIANT,
	) -> HRESULT,

	fn create_instance_2(
		type_name: BSTR, 
		ignore_case: VARIANT_BOOL, 
		p_ret_val: *mut VARIANT, 
	) -> HRESULT, 

	fn create_instance_3(
		type_name: BSTR, 
		ignore_case: VARIANT_BOOL,
		binding_attr: BindingFlags, 
		binder: *mut _Binder, 
		args: *mut SAFEARRAY, 
		culture: *mut _CultureInfo, 
		activation_attributes: *mut SAFEARRAY,
		p_ret_val: *mut VARIANT,
	) -> HRESULT, 

	fn get_loaded_modules(
		p_ret_val: *mut *mut SAFEARRAY, 
	) -> HRESULT, 

	fn get_loaded_modules_2(
		get_resource_modules: VARIANT_BOOL, 
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT, 

	fn get_modules(
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT, 

	fn get_modules_2(
		get_resource_modules: VARIANT_BOOL, 
		p_ret_val: *mut *mut SAFEARRAY, 
	) -> HRESULT, 

	fn get_module(
		name: BSTR, 
		p_ret_val: *mut *mut _Module,
	) -> HRESULT, 

	fn get_referenced_assemblies(
		p_ret_val: *mut *mut SAFEARRAY,
	) -> HRESULT, 

	fn get_global_assembly_cache(
		p_ret_val: *mut VARIANT_BOOL,
	) -> HRESULT,
}}



RIDL!{#[uuid(0x8a7c1442, 0xa9fb, 0x366b, 0x80, 0xd8, 0x49, 0x39, 0xff, 0xa6, 0xdb, 0xe0)]
interface _FieldInfo(FieldInfoVtbl): IUnknown(IUnknownVtbl){
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
    p_except_info: c_long, 
    pu_arg_err: c_long,
  ) -> HRESULT,

  fn to_string(
    p_ret_val: *mut BSTR,
  ) -> HRESULT,

  fn equals(
    other: VARIANT, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT, 

  fn hash_code(
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
    attribute_type: *const _Type,
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_custom_attributes_2(
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn is_defined(
    attribute_type: *const _Type, 
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_field_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn get_value(
    obj: VARIANT, 
    p_ret_val: *const VARIANT,
  ) -> HRESULT,

  fn get_value_direct(
    obj: VARIANT, 
    p_ret_val: *const VARIANT,
  ) -> HRESULT,

  fn set_value(
    obj: VARIANT, 
    val: VARIANT, 
    invoke_attr: BindingFlags,
    binder: *const _Binder, 
    culture: *const _CultureInfo,
  ) -> HRESULT,

  fn set_value_direct(
    obj: VARIANT, 
    val: VARIANT,
  ) -> HRESULT,

  fn get_field_handle(
    p_ret_val: *mut RuntimeFieldHandle,
  ) -> HRESULT,

  fn get_attributes(
    p_ret_val: *mut FieldAttributes,
  ) -> HRESULT,

  fn set_value_2(
    obj: VARIANT,
    val: VARIANT,
  ) -> HRESULT,

  fn get_is_public(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_private(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_family(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_assembly(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_family_and_assembly(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_family_or_assembly(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_static(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_init_only(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_literal(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_serialized(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_special_name(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_p_invoke_impl(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

}}

RIDL!{#[uuid(0xf59ed4e4, 0xe68f, 0x3218, 0xbd, 0x77, 0x06, 0x1a, 0xa8, 0x28, 0x24, 0xbf)]
interface _PropertyInfo(_PropertyInfoVtbl): IUnknown(IUnknownVtbl){
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
    p_except_info: c_long, 
    pu_arg_err: c_long,
  ) -> HRESULT,

  fn to_string(
    p_ret_val: *mut BSTR,
  ) -> HRESULT,

  fn equals(
    other: VARIANT, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT, 

   fn hash_code(
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
    attribute_type: *const _Type,
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_custom_attributes_2(
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn is_defined(
    attribute_type: *const _Type, 
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_property_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn get_value(
    obj: VARIANT, 
    index: *const SAFEARRAY,
    p_ret_val: *mut VARIANT,
  ) -> HRESULT,

  fn get_value_2(
    obj: VARIANT, 
    invoke_attr: BindingFlags,
    binder: *const _Binder,
    index: *const SAFEARRAY,
    culture: *const _CultureInfo,
    p_ret_val: *mut VARIANT,
  ) -> HRESULT,

  fn set_value(
    obj: VARIANT, 
    val: VARIANT, 
    index: *const SAFEARRAY,
  ) -> HRESULT,

  fn set_value_2(
    obj: VARIANT, 
    val: VARIANT, 
    invoke_attr: BindingFlags, 
    binder: *const _Binder,
    index: *const SAFEARRAY, 
    culture: *const _CultureInfo,
  ) -> HRESULT,

  fn get_accessors(
    non_public: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_get_method(
    non_public: VARIANT_BOOL, 
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_set_method(
    non_public: VARIANT_BOOL, 
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_index_params(
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_attributes(
    p_ret_val: *mut PropertyAttributes,
  ) -> HRESULT,

  fn get_can_read(
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn get_can_write(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_accessors_2(
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_get_method_2(
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_set_method_2(
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_is_special_name(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

}}


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



type FExecuteInAppDomainCallback = Fn(*mut c_void)->HRESULT;

RIDL!{#[uuid(0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02)]
interface ICLRRuntimeHost(ICLRRuntimeHostVtbl): IUnknown(IUnknownVtbl) {
	fn start() -> HRESULT,
	fn stop() -> HRESULT,
	fn set_host_control(
		host_control: *mut IHostControl,
	) -> HRESULT, 
	fn get_clr_control(
		clr_control: *mut *mut ICLRControl, 
	) -> HRESULT,
	fn unload_app_domain(
		app_domain_id: DWORD, 
		wait_until_done: bool,
	) -> HRESULT,
	fn execute_in_app_domain(
		app_domain_id: DWORD, 
		callback: *mut FExecuteInAppDomainCallback, 
		cookie: *mut c_void,
	) -> HRESULT,
	fn get_current_app_domain_id(
		app_domain_id: *mut DWORD,
	) -> HRESULT,
	fn execute_application(
		app_full_name: LPCWSTR, 
		dw_manifest_paths: DWORD, 
		manifest_paths: *mut LPCWSTR, 
		dw_activation_data: DWORD, 
		activation_data: *mut LPCWSTR, 
		return_value: *mut c_int, 
	) -> HRESULT,
	fn execute_in_default_app_domain(
		assembly_path: LPCWSTR, 
		type_name: LPCWSTR, 
		argument: LPCWSTR, 
		return_value: *mut DWORD,
	) -> HRESULT,
}}


//self
RIDL!{#[uuid(0xbca8b44d, 0xaad6, 0x3a86, 0x8a, 0xb7, 0x03, 0x34, 0x9f, 0x4f, 0x2d, 0xa2)]
interface _Type(_TypeVtbl): IUnknown(IUnknownVtbl){
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
    p_except_info: c_long, 
    pu_arg_err: c_long,
  ) -> HRESULT,
  
  fn to_string(
    p_ret_val: *mut BSTR,
  ) -> HRESULT,
  
  fn equals(
    other: VARIANT, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,
  
  fn hash_code(
    p_ret_val: *mut c_long,
  ) -> HRESULT, 
  
  fn get_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT, 

  fn get_member_type(
    p_ret_val: *mut MemberTypes,
  ) -> HRESULT,

  fn get_name(
    p_ret_val: *const BSTR,
  ) -> HRESULT,

  fn get_declaring_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn get_reflected_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn get_custom_attributes(
    attribute_type: *const _Type, 
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_custom_attributes_2(
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn is_defined(
    attribute_type: *const _Type, 
    inherit: VARIANT_BOOL, 
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn get_guid(
    p_ret_val: *mut GUID, 
  ) -> HRESULT, 

  fn get_module(
    p_ret_val: *mut *mut _Module, 
  ) -> HRESULT,

  fn get_assembly(
    p_ret_val: *mut *mut _Assembly,
  ) -> HRESULT,

  fn get_type_handle(
    p_ret_val: *mut RuntimeTypeHandle, 
  ) -> HRESULT,

  fn get_full_name(
    p_ret_val: *mut BSTR, 
  ) -> HRESULT, 

  fn get_namespace(
    p_ret_val: *mut BSTR, 
  ) -> HRESULT, 

  fn get_assembly_qualified_name(
    p_ret_val: *mut BSTR,
  ) -> HRESULT,

  fn get_array_rank(
    p_ret_val: *mut c_long,
  ) -> HRESULT,

  fn get_base_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn get_constructors(
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_interface(
    name: BSTR, 
    ignore_case: VARIANT_BOOL, 
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT, 

  fn get_interfaces(
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn find_interfaces(
    filter: *const _TypeFilter, 
    filter_criteria: VARIANT, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT, 

  fn get_event(
    name: BSTR, 
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut _EventInfo, 
  ) -> HRESULT, 

  fn get_events(
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_events_2(
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_nested_types(
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT, 

  fn get_nested_type(
    name: BSTR, 
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT, 

  fn get_member(
    name: BSTR, 
    member_type: MemberTypes, 
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_default_members(
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn find_members(
    member_type: MemberTypes,
    binding_attr: BindingFlags,
    filter: *const _MemberFilter, 
    filter_criteria: VARIANT, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_element_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn is_subclass_of(
    c: *const _Type, 
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn is_instance_of_type(
    o: VARIANT, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn is_assignable_from(
    c: *const _Type, 
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_interface_map(
    interface_type: *mut _Type,
    p_ret_val: *mut InterfaceMapping,
  ) -> HRESULT, 

  fn get_method(
    name: BSTR, 
    binding_attr: BindingFlags, 
    binder: *const _Binder, 
    types: *const SAFEARRAY, 
    modifiers: *const SAFEARRAY, 
    p_ret_val: *mut *mut _MethodInfo, 
  ) -> HRESULT, 

  fn get_method_2(
    name: BSTR, 
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_methods(
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT, 

  fn get_field(
    name: BSTR, 
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut _FieldInfo, 
  ) -> HRESULT,

  fn get_fields(
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_property(
    name: BSTR, 
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut _PropertyInfo,
  ) -> HRESULT,

  fn get_property_2(
    name: BSTR, 
    binding_attr: BindingFlags,
    binder: *const _Binder,
    return_type: *const _Type,
    types: *const SAFEARRAY,
    modifiers: *const SAFEARRAY,
    p_ret_val: *mut *mut _PropertyInfo,
  ) -> HRESULT,

  fn get_properties(
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_member_2(
    name: BSTR,
    binding_attr: BindingFlags, 
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_members(
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn invoke_member(
    name: BSTR, 
    invoke_attr: BindingFlags, 
    binder: *const _Binder, 
    target: VARIANT, 
    args: *const SAFEARRAY, 
    modifiers: *const SAFEARRAY, 
    culture: *const _CultureInfo,
    named_parameters: *const SAFEARRAY, 
    p_ret_val: *mut VARIANT,
  ) -> HRESULT, 

  fn get_underlying_system_type(
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn invoke_member_2(
    name: BSTR, 
    invoke_attr: BindingFlags, 
    binder: *const _Binder, 
    target: VARIANT, 
    args: *const SAFEARRAY, 
    culture: *const _CultureInfo,
    p_ret_val: *mut VARIANT, 
  ) -> HRESULT,

  fn invoke_member_3(
    name: BSTR, 
    invoke_attr: BindingFlags,
    binder: *const _Binder, 
    target: VARIANT, 
    args: *const SAFEARRAY, 
    p_ret_val: *mut VARIANT, 
  ) -> HRESULT,

  fn get_constructor(
    binding_attr: BindingFlags, 
    binder: *const _Binder, 
    call_convention: CallingConventions, 
    types: *const SAFEARRAY, 
    modifiers: *const SAFEARRAY, 
    p_ret_val: *mut *mut _ConstructorInfo, 
  ) -> HRESULT, 

  fn get_constructor_2(
    binding_attr: BindingFlags, 
    binder: *const _Binder, 
    types: *const SAFEARRAY, 
    modifiers: *const SAFEARRAY, 
    p_ret_val: *mut *mut _ConstructorInfo, 
  ) -> HRESULT, 

  fn get_constructor_3(
    types: *const SAFEARRAY, 
    p_ret_val: *mut *mut _ConstructorInfo, 
  ) -> HRESULT, 

  fn get_constructors_2(
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_type_initializer(
    p_ret_val: *mut *mut _ConstructorInfo,
  ) -> HRESULT,

  fn get_method_3(
    name: BSTR, 
    binding_attr: BindingFlags, 
    binder: *const _Binder, 
    call_convention: CallingConventions, 
    types: *const SAFEARRAY, 
    modifiers: *const SAFEARRAY,
    p_ret_val: *mut *mut _MethodInfo, 
  ) -> HRESULT,

  fn get_method_4(
    name: BSTR, 
    types: *const SAFEARRAY, 
    modifiers: *const SAFEARRAY, 
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_method_5(
    name: BSTR, 
    types: *const SAFEARRAY, 
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_method_6(
    name: BSTR, 
    p_ret_val: *mut *mut _MethodInfo,
  ) -> HRESULT,

  fn get_methods_2(
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_field_2(
    name: BSTR, 
    p_ret_val: *mut *mut _FieldInfo,
  ) -> HRESULT,

  fn get_fields_2(
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT, 

  fn get_interface_2(
    name: BSTR, 
    p_ret_val: *mut *mut _Type, 
  ) -> HRESULT,

  fn get_event_2(
    name: BSTR, 
    p_ret_val: *mut *mut _EventInfo, 
  ) -> HRESULT, 

  fn get_property_3(
    name: BSTR, 
    return_type: *const _Type,
    types: *const SAFEARRAY,
    modifiers: *const SAFEARRAY, 
    p_ret_val: *mut *mut _PropertyInfo,  
  ) -> HRESULT,

  fn get_property_4(
    name: BSTR, 
    return_type: *const _Type,
    types: *const SAFEARRAY, 
    p_ret_val: *mut *mut _PropertyInfo,
  ) -> HRESULT,

  fn get_property_5(
    name: BSTR, 
    types: *const SAFEARRAY, 
    p_ret_val: *mut *mut _PropertyInfo,
  ) -> HRESULT, 

  fn get_property_6(
    name: BSTR, 
    return_type: *const _Type, 
    p_ret_val: *mut *mut _PropertyInfo,
  ) -> HRESULT, 

  fn get_property_7(
    name: BSTR, 
    p_ret_val: *mut *mut _PropertyInfo,
  ) -> HRESULT, 

  fn get_properties_2(
    binding_attr: BindingFlags,
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_nested_types_2(
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT, 

  fn get_nested_type_2(
    name: BSTR, 
    p_ret_val: *mut *mut _Type,
  ) -> HRESULT,

  fn get_member_3(
    name: BSTR, 
    p_ret_val: *mut *mut SAFEARRAY, 
  ) -> HRESULT,

  fn get_members_2(
    p_ret_val: *mut *mut SAFEARRAY,
  ) -> HRESULT,

  fn get_attributes(
    p_ret_val: *mut TypeAttributes, 
  ) -> HRESULT,

  fn get_is_not_public(
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn get_is_public(
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT, 

  fn get_is_nested_public(
    p_ret_val: *mut VARIANT_BOOL,  
  ) -> HRESULT,

  fn get_is_nested_private(
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn get_is_nested_family(
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn get_is_nested_assembly(
    p_ret_val: *mut VARIANT_BOOL, 
  ) -> HRESULT,

  fn get_is_nested_fam_and_assem(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_nested_fam_or_assem(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_auto_layout(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_layout_sequential(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_explicit_layout(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_class(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_interface(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_value_type(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_abstract(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_sealed(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_enum(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_special_name(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_import(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,
  
  fn get_is_serializeable(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_ansi_class(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_unicode_class(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,
  
  fn get_is_auto_class(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_array(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_by_ref(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_pointer(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_primitive(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_com_object(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_has_element_type(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn get_is_contextful(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,
  
  fn get_is_marshal_by_ref(
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,

  fn equals_2(
    o: *mut _Type,
    p_ret_val: *mut VARIANT_BOOL,
  ) -> HRESULT,
  
}}

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