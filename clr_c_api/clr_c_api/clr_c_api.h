#pragma once

#ifdef __cplusplus

#pragma comment(lib, "mscoree.lib")

#import "mscorlib.tlb" raw_interfaces_only no_smart_pointers \
    high_property_prefixes("_get","_put","_putref")		\
    rename("ReportEvent", "InteropServices_ReportEvent") \
	rename("or", "ORR")

#include <comdef.h>

using namespace mscorlib;

#import "RustAppDomainManager.tlb" raw_interfaces_only no_smart_pointers
using namespace RustAppDomainManager;
#define RUST_APP_DOMAIN_MANAGER_GUID_STR "B47320A6-6265-4C34-90AC-3FF2A909686C"
DEFINE_GUID(RUST_APP_DOMAIN_MANAGER_GUID,
	0xB47320A6, 0x6265, 0x4C34, 0x90, 0xAC, 0x3F, 0xF2, 0xA9, 0x09, 0x68, 0x6C);



#define RUST_HOST_CONTROL_GUID_STR "1E20D486-67C7-4CD6-B56B-41D2297D5B2F"
DEFINE_GUID(RUST_HOST_CONTROL_GUID,
	0x1e20d486, 0x67c7, 0x4cd6, 0xb5, 0x6b, 0x41, 0xd2, 0x29, 0x7d, 0x5b, 0x2f);


extern "C" {
#endif

	class __declspec(uuid(RUST_HOST_CONTROL_GUID_STR)) IRustHostControl : public IHostControl {
	public:
		virtual ICustomAppDomainManager* GetDomainManager() = 0;
	};

	inline wchar_t* string_to_lpcwstr(const char* text) {
		int iLen = MultiByteToWideChar(CP_ACP, 0, text, -1, 0, 0);
		BSTR bs_text = SysAllocStringLen(0, iLen);
		MultiByteToWideChar(CP_ACP, 0, text, -1, bs_text, iLen);
		return bs_text;
	}
	
	IRustHostControl* RustHostControl_new();

#ifdef __cplusplus
}
#endif

class RustHostControl : public IRustHostControl {
public:
	HRESULT __stdcall QueryInterface(const IID &iid, void **ppv);
	ULONG __stdcall AddRef();
	ULONG __stdcall Release();
	HRESULT __stdcall GetHostManager(REFIID id, void **ppHostManager);
	HRESULT __stdcall SetAppDomainManager(DWORD dwAppDomainID, IUnknown* pUnkAppDomainManager);
	RustHostControl();
	virtual ~RustHostControl();
	virtual ICustomAppDomainManager* GetDomainManager();
private:
	long refCount_m;
	ICustomAppDomainManager* defaultDomainManager_m;
};
