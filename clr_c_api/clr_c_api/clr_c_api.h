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

class SamplehostControl : IHostControl {
public:
	SamplehostControl();
	virtual ~SamplehostControl();
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
	};

	typedef struct ICLRMetaHost ICLRMetaHost;
	typedef struct ICLRRuntimeInfo ICLRRuntimeInfo;
	typedef struct ICLRRuntimeHost ICLRRuntimeHost;
	typedef struct ICorRuntimeHost ICorRuntimeHost;
	//typedef struct _Assembly _Assembly;

	CAPIResult newCLRMetaHost();
	CAPIResult freeCLRMetaHost(ICLRMetaHost* host);
	CAPIResult CLRMetaHost_get_runtime(ICLRMetaHost* host, const char* version);
	CAPIResult CLRRuntimeInfo_free(ICLRRuntimeInfo* info);
	bool CLRRuntimeInfo_is_loadable(ICLRRuntimeInfo* info);
	CAPIResult CLRRuntimeInfo_get_clr_runtime(ICLRRuntimeInfo* info);
	CAPIResult CLRRuntimeHost_stop(ICLRRuntimeHost* host);
	bool CLRRuntimeHost_start(ICLRRuntimeHost* host);
	CAPIResult CLRRuntimeHost_load_assembly(ICLRRuntimeInfo * info, ICLRRuntimeHost* host, char* assemblyName);
	CAPIResult Assembly_release(_Assembly* assembly);
#ifdef __cplusplus
}
#endif