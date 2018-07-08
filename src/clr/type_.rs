//type_.rs
#![allow(dead_code)]
#![allow(non_snake_case)]
//std

//3rd party
use winapi::ctypes::{c_long, c_short};

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL, VARTYPE, VT_UNKNOWN};

use winapi::um::oaidl::{VARIANT};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use winapi::um::oaidl::{SAFEARRAY};

//self
use clr::c_api::{ClrWrapper};
use clr::field::MemberTypes;
use clr::misc::{TypeAttributes, _ConstructorInfo, CallingConventions, _CultureInfo, _Module, 
RuntimeTypeHandle, BindingFlags, _TypeFilter, _EventInfo, 
_MemberFilter, InterfaceMapping, _Binder};

use clr::method::{_MethodInfo};
use clr::field::{_FieldInfo, _PropertyInfo};

use clr::bindings::_Assembly;

pub struct Type {
  ptr: *mut _Type
}

impl ClrWrapper for Type {
  type InnerPointer = _Type;
  
  fn into_raw(&self) -> *mut Self::InnerPointer {
    self.ptr
  }
	fn vartype() -> VARTYPE {
    VT_UNKNOWN as u16
  }

}

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
