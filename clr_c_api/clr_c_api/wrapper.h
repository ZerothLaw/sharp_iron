#pragma once

#ifdef __cplusplus
#pragma comment(lib, "mscoree.lib")
#import <mscorlib.tlb> high_property_prefixes("_get","_put","_putref") rename("value","val") rename("or", "ORR") rename("ReportEvent", "InteropServices_ReportEvent") raw_interfaces_only no_smart_pointers

using namespace mscorlib;
extern "C" {
#endif

	__declspec(dllexport)
	typedef struct ICLRMetaHost ICLRMetaHost;
	typedef struct ICLRRuntimeInfo ICLRRuntimeInfo;
	typedef struct ICLRRuntimeHost ICLRRuntimeHost;
	typedef struct ICorRuntimeHost ICorRuntimeHost;

	__declspec(dllexport)
	ICLRMetaHost* newCLRMetaHost();
	ICLRRuntimeInfo* CLRMetaHost_get_runtime(ICLRMetaHost* host, char* version);
	bool CLRRuntimeInfo_is_loadable(ICLRRuntimeInfo* info);
	ICLRRuntimeHost* CLRRuntimeInfo_get_clr_runtime(ICLRRuntimeInfo* info);
	ICorRuntimeHost* CLRRuntimeInfo_get_cor_runtime(ICLRRuntimeInfo* info);
	_AppDomain* CorRuntimeHost_get_app_domain(ICorRuntimeHost* host);
	_Assembly* AppDomain_load_assembly(_AppDomain* appDomain, char* assemblyName);
	//variant_t CLRRuntimeHost_instantiate_class(char* className);


#ifdef __cplusplus
}
#endif