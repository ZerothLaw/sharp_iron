//c_api module

use winapi::ctypes::{c_void, c_char};
use winapi::shared::winerror::{HRESULT};

#[repr(C)]
pub struct CAPIResult {
	pub hr: HRESULT, 
	pub ok: bool, 
	pub c_ptr: *mut c_void, 
	pub ws_ptr: *mut u16
}

type CLRMetaHost = *mut c_void;

extern "C" {
	//CLRMetaHost functions
	pub fn CLRMetaHost_new()                                                  -> CAPIResult;
	pub fn CLRMetaHost_get_runtime(host: *mut c_void, version: *const c_char) -> CAPIResult; 
	
	//CLRRuntimeInfo functions
	pub fn CLRRuntimeInfo_is_loadable(           runtime_info: *mut c_void)                              -> bool;
	pub fn CLRRuntimeInfo_is_loaded(             runtime_info: *mut c_void)                              -> bool;
	pub fn CLRRuntimeInfo_is_loaded_from_handle( runtime_info: *mut c_void, process_handle: *mut c_void) -> bool;
	pub fn CLRRuntimeInfo_is_started(            runtime_info: *mut c_void)                              -> bool;
	pub fn CLRRuntimeInfo_get_clr_runtime(       runtime_info: *mut c_void)                              -> CAPIResult;
	pub fn CLRRuntimeInfo_load_error_string(     runtime_info: *mut c_void, hr: HRESULT)                 -> CAPIResult;
	pub fn CLRRuntimeInfo_load_library(          runtime_info: *mut c_void, dll_name: *const c_char)     -> CAPIResult;
	
	//utility method
	pub fn CAPI_free_error_string(error_string: *mut c_char) -> CAPIResult;
	
	//CLRRuntimeHost functions
	pub fn CLRRuntimeHost_start(         clr_host: *mut c_void)        -> bool;
	pub fn CLRRuntimeHost_stop(          clr_host: *mut c_void)        -> CAPIResult;
	pub fn CLRRuntimeHost_load_assembly( clr_host: *mut c_void, 
	                                     clr_info: *mut c_void, 
										 assembly_name: *const c_char) -> CAPIResult;
	
	//Assembly functions
	pub fn Assembly_release(  assembly: *mut c_void) -> CAPIResult;
	pub fn Assembly_get_type( assembly: *mut c_void) -> CAPIResult;
	
	//Type functions
	pub fn Type_call_static_method( type_: *mut c_void, 
	                                method_name: *const c_char) -> CAPIResult;
	pub fn Type_create_instance(    type_: *mut c_void)         -> CAPIResult;
	
	//Object instance functions
	pub fn Object_invoke( obj: *mut c_void, 
	                      method_name: *const c_char) -> CAPIResult;
	pub fn Object_free(   obj: *mut c_void)           -> CAPIResult;
}
