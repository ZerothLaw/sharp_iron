//assembly.rs
//std

//3rd party
use winapi::Interface;
use winapi::ctypes::{c_long, c_short};
use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::winerror::{HRESULT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};


//self
use clr::type_::_Type;
use clr::method::_MethodInfo;
use clr::misc::{_Binder, _CultureInfo, _Evidence, _FileStream, _ManifestResourceInfo, _ModuleResolveEventHandler, _Module, 
_Stream, _SerializationInfo, _Version, BindingFlags, StreamingContext, };

//body
#[repr(C)]
pub enum AssemblyBuilderAccess
{
    Run = 1,
    Save = 2,
    RunAndSave = 3,
    ReflectionOnly = 6,
    RunAndCollect = 9
}
add_uuid!(AssemblyBuilderAccess, 0xf0778630, 0xac34, 0x3d71, 0x9f, 0xab, 0x61, 0x7f, 0x61, 0x24, 0x30, 0x65);


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
	fn type_(
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
