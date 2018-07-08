//std
#![allow(dead_code)]
#![allow(non_snake_case)]
//3rd party
use winapi::Interface;

use winapi::shared::guiddef::{GUID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::ctypes::{c_short, c_long};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{VARIANT};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use winapi::um::oaidl::{SAFEARRAY};

//self
use clr::type_::_Type;
use clr::misc::{_Binder, BindingFlags, _CultureInfo, RuntimeFieldHandle, FieldAttributes};
use clr::method::{_MethodInfo};


//body
#[repr(C)]
pub enum MemberTypes {
    Constructor = 1,
    Event = 2,
    Field = 4,
    Method = 8,
    Property = 16,
    TypeInfo = 32,
    Custom = 64,
    NestedType = 128,
    All = 191
}
add_uuid!(MemberTypes, 0x513b8b77, 0x4930, 0x36ba, 0x9a, 0x22, 0x0d, 0xae, 0xb2, 0x93, 0xe1, 0x09);

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

#[repr(C)]
pub enum PropertyAttributes
{
    None = 0,
    SpecialName = 512,
    ReservedMask = 62464,
    RTSpecialName = 1024,
    HasDefault = 4096,
    Reserved2 = 8192,
    Reserved3 = 16384,
    Reserved4 = 32768
}
add_uuid!(PropertyAttributes, 0x816c979c, 0xd3d2, 0x3101, 0xb5, 0xca, 0xe4, 0xa5, 0xc5, 0xe9, 0x66, 0xfa);