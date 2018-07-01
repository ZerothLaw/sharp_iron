//assembly.rs
//std

//3rd party
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
RIDL!{#[uuid(0xb42b6aac, 0x317e, 0x34d5, 0x9f, 0xa9, 0x09, 0x3b, 0xb4, 0x16, 0x0c, 0x50)]
interface _AssemblyName(_AssemblyNameVtbl): IUnknown(IUnknownVtbl) {
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

RIDL!{#[uuid(0x17156360, 0x2f1a, 0x384a, 0xbc, 0x52, 0xfd, 0xe9, 0x3c, 0x21, 0x5c, 0x5b)]
interface _Assembly(_AssemblyVtbl): IDispatch(IDispatchVtbl){
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
	fn get_CodeBase(
		pRetVal: *mut BSTR,
	) -> HRESULT,
	fn get_EscapedCodeBase(
		pRetVal: *mut BSTR,
	) -> HRESULT,
	fn GetName(
		pRetVal: *mut *mut _AssemblyName,
	) -> HRESULT, 
	fn GetName_2(
		copiedName: VARIANT_BOOL, 
		pRetVal: *mut *mut _AssemblyName,
	) -> HRESULT, 
	fn get_FullName(
		pRetVal: *mut BSTR, 
	) -> HRESULT, 
	fn get_EntryPoint(
		pRetVal: *mut *mut _MethodInfo, 
	) -> HRESULT,
	fn GetType_2(
		name: BSTR, 
		pRetVal: *mut *mut _Type, 
	) -> HRESULT, 
	fn GetType_3(
		name: BSTR, 
		throwOnError: VARIANT_BOOL, 
		pRetVal: *mut *mut _Type,
	) -> HRESULT,

	fn GetExportedTypes(
		pRetVal: *mut *mut SAFEARRAY, 
	) -> HRESULT,
	
	fn GetTypes(
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn GetManifestResourceStream(
		Type: *mut _Type, 
		name: BSTR, 
		pRetVal: *mut *mut _Stream,
	) -> HRESULT, 

	fn GetManifestResourceStream_2(
		name: BSTR, 
		pRetVal: *mut *mut _Stream,
	) -> HRESULT,

	fn GetFile(
		name: BSTR, 
		pRetVal: *mut *mut _FileStream,
	) -> HRESULT,

	fn GetFiles(
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn GetFiles_2(
		getResourceModules: VARIANT_BOOL, 
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn GetManifestResourceNames(
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT,

	fn GetManifestResourceInfo(
		resourceName: BSTR, 
		pRetVal: *mut *mut _ManifestResourceInfo,
	) -> HRESULT,

	fn get_Location(
		pRetVal: *mut BSTR, 
	) -> HRESULT, 

	fn get_Evidence(
		pRetVal: *mut *mut _Evidence,
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

	fn GetObjectData(
		info: *mut _SerializationInfo, 
		Context: StreamingContext,
	) -> HRESULT,

	fn add_ModuleResolve(
		val: *mut _ModuleResolveEventHandler, 
	) -> HRESULT, 

	fn remove_ModuleResolve(
		val: *mut _ModuleResolveEventHandler,
	) -> HRESULT, 

	fn GetType_4(
		name: BSTR, 
		throwOnError: VARIANT_BOOL, 
		ignoreCase: VARIANT_BOOL, 
		pRetVal: *mut *mut _Type,
	) -> HRESULT, 

	fn GetSatelliteAssembly(
		culture: *mut _CultureInfo, 
		pRetVal: *mut *mut _Assembly, 
	) -> HRESULT, 

	fn GetSatelliteAssembly_2(
		culture: *mut _CultureInfo, 
		Version: *mut _Version, 
		pRetVal: *mut *mut _Assembly,
	) -> HRESULT,

	fn LoadModule(
		moduleName: BSTR, 
		rawModule: *mut SAFEARRAY, 
		pRetVal: *mut *mut _Module, 
	) -> HRESULT, 

	fn LoadModule_2(
		moduleName: BSTR, 
		rawModule: *mut SAFEARRAY, 
		rawSymbolStore: *mut SAFEARRAY, 
		pRetVal: *mut *mut _Module,
	) -> HRESULT,

	fn CreateInstance(
		typeName: BSTR, 
		pRetVal: *mut VARIANT,
	) -> HRESULT,

	fn CreateInstance_2(
		typeName: BSTR, 
		ignoreCase: VARIANT_BOOL, 
		pRetVal: *mut VARIANT, 
	) -> HRESULT, 

	fn CreateInstance_3(
		typeName: BSTR, 
		ignoreCase: VARIANT_BOOL,
		bindingAttr: BindingFlags, 
		Binder: *mut _Binder, 
		args: *mut SAFEARRAY, 
		culture: *mut _CultureInfo, 
		activationAttributes: *mut SAFEARRAY,
		pRetVal: *mut VARIANT,
	) -> HRESULT, 

	fn GetLoadedModules(
		pRetVal: *mut *mut SAFEARRAY, 
	) -> HRESULT, 

	fn GetLoadedModules_2(
		getResourceModules: VARIANT_BOOL, 
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT, 

	fn GetModules(
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT, 

	fn GetModules_2(
		getResourceModules: VARIANT_BOOL, 
		pRetVal: *mut *mut SAFEARRAY, 
	) -> HRESULT, 

	fn GetModule(
		name: BSTR, 
		pRetVal: *mut *mut _Module,
	) -> HRESULT, 

	fn GetReferencedAssemblies(
		pRetVal: *mut *mut SAFEARRAY,
	) -> HRESULT, 

	fn get_GlobalAssemblyCache(
		pRetVal: *mut VARIANT_BOOL,
	) -> HRESULT,
}}
