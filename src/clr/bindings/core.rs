//std

//3rd party
use winapi::ctypes::{c_long, c_short};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};


//self
use clr::type_::_Type;
use clr::misc::_Evidence;
use clr::misc::_EventHandler;
use clr::misc::IPrincipal;
use clr::misc::_PolicyLevel;
use clr::object_handle::{_ObjectHandle};
use clr::misc::_CultureInfo;
use clr::misc::_Binder;
use clr::misc::BindingFlags;

use clr::misc::{_FileStream, 
                _ManifestResourceInfo, _ModuleResolveEventHandler, _Module, 
                _Stream, _SerializationInfo, _Version,
				StreamingContext,};

use clr::method::_MethodInfo;


use clr::misc::_UnhandledExceptionEventHandler;
use clr::misc::_ResolveEventHandler;
use clr::misc::_AssemblyLoadEventHandler;

use clr::bindings::_AssemblyBuilder;
use clr::bindings::_CrossAppDomainDelegate;
use clr::bindings::_PermissionSet;

use clr::bindings::AssemblyBuilderAccess;
use clr::bindings::PrincipalPolicy;

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
