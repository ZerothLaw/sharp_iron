#pragma once

#ifdef __cplusplus
#pragma comment(lib, "mscoree.lib")
#import <mscorlib.tlb> high_property_prefixes("_get","_put","_putref") rename("value","val") rename("or", "ORR") rename("ReportEvent", "InteropServices_ReportEvent") raw_interfaces_only no_smart_pointers

using namespace mscorlib;

#define FACADE_GUID_STR "C5774F7F-8E4F-477D-BA87-F27429D7CB46"
DEFINE_GUID(FACADE_GUID ,
	0xc5774f7f, 0x8e4f, 0x477d, 0xba, 0x87, 0xf2, 0x74, 0x29, 0xd7, 0xcb, 0x46);

struct __declspec(uuid(FACADE_GUID_STR)) FacadeInterface;
struct FacadeInterface : IUnknown {
	virtual HRESULT __stdcall Run(LPWSTR name, LPWSTR* result) = 0;
};

class FacadeHostControl : IHostControl {
public:
	FacadeHostControl();
	virtual ~FacadeHostControl();
	FacadeInterface* GetFacade();
	HRESULT __stdcall QueryInterface(const IID &iid, void **ppv);
	ULONG __stdcall AddRef();
	ULONG __stdcall Release();
	HRESULT __stdcall GetHostManager(REFIID id, void **ppHostManager);
	HRESULT __stdcall SetAppDomainManager(DWORD dwAppDomainID, IUnknown* pUnkAppDomainManager);

private:
	long refCount_m;
	FacadeInterface* defaultDomainManager_m;
};

extern "C" {
#endif

	struct CAPIResult {
		HRESULT hr; 
		bool ok;
		void* c_ptr;
		wchar_t* ws_ptr;
	};

	typedef struct ICLRMetaHost ICLRMetaHost;
	typedef struct ICLRRuntimeInfo ICLRRuntimeInfo;
	typedef struct ICLRRuntimeHost ICLRRuntimeHost;
	typedef struct ICorRuntimeHost ICorRuntimeHost;
	typedef struct ICLRObject ICLRObject;
	typedef struct ICLRType ICLRType;
	//typedef struct _Assembly _Assembly;


	//ICLRMetaHost calls
	CAPIResult CLRMetaHost_new();
	CAPIResult CLRMetaHost_get_runtime(ICLRMetaHost* host, const char* version);

	//ICLRRuntimeInfo calls
	bool       CLRRuntimeInfo_is_loadable(          ICLRRuntimeInfo* info);
	bool       CLRRuntimeInfo_is_loaded(            ICLRRuntimeInfo* info);
	bool       CLRRuntimeInfo_is_loaded_from_handle(ICLRRuntimeInfo* info, HANDLE* processHandle);
	bool       CLRRuntimeInfo_is_started(           ICLRRuntimeInfo *info);
	CAPIResult CLRRuntimeInfo_get_clr_runtime(      ICLRRuntimeInfo* info);
	CAPIResult CLRRuntimeInfo_load_error_string(    ICLRRuntimeInfo* info, HRESULT hr);
	CAPIResult CLRRuntimeInfo_load_library(         ICLRRuntimeInfo* info, const char* dllName);

	//Utility method, to pair with CLRRuntimeInfo_load_error_string
	CAPIResult CAPI_free_error_string(const char* err_string);

	//ICLRRuntimeHost calls
	CAPIResult CLRRuntimeHost_stop(         ICLRRuntimeHost* host);
	bool       CLRRuntimeHost_start(        ICLRRuntimeHost* host);
	CAPIResult CLRRuntimeHost_load_assembly(ICLRRuntimeHost* host, ICLRRuntimeInfo * info, const char* assemblyName);

	//Assembly calls
	CAPIResult Assembly_release( _Assembly* assembly);
	CAPIResult Assembly_get_type(_Assembly* assembly, const char* typeName);
	
	//Type calls
	CAPIResult Type_call_static_method(_Type* type, const char* methodName);
	CAPIResult Type_create_instance(   _Type* type, void* args);

	//.Net object instance calls
	CAPIResult Object_invoke(void* vtObj, const char* methodName);
	CAPIResult Object_free(  void* vtObj);
#ifdef __cplusplus
}
#endif