// clr_c_api.cpp : Defines the exported functions for the DLL application.
//

#include "stdafx.h"
#include "wrapper.h"
#include <mscoree.h>
#import <mscorlib.tlb> rename("value","val") rename("or", "ORR") rename("ReportEvent", "InteropServices_ReportEvent") raw_interfaces_only no_smart_pointers

extern "C" {
	ICLRMetaHost* newCLRMetaHost() {
		ICLRMetaHost *pMetaHost = NULL;
		HRESULT hr;
		hr = CLRCreateInstance(CLSID_CLRMetaHost, IID_ICLRMetaHost, (LPVOID*)&pMetaHost);
		if (!FAILED(hr)) {
			return pMetaHost;
		}
		return NULL;
	}

	ICLRRuntimeInfo* CLRMetaHost_get_runtime(ICLRMetaHost* host, char* version) {
		if (host != NULL) {
			ICLRRuntimeInfo *pRuntimeInfo = NULL;
			HRESULT hr = host->GetRuntime((PCWSTR)version, IID_ICLRRuntimeInfo, (LPVOID*)&pRuntimeInfo);
			if (!FAILED(hr)) {
				return pRuntimeInfo;
			}
		}
		return NULL;
	}

	bool CLRRuntimeInfo_is_loadable(ICLRRuntimeInfo * info)
	{
		if (info != NULL) {
			BOOL bLoadable;
			HRESULT hr = info->IsLoadable(&bLoadable);
			return bLoadable;
		}
		return false;
	}

	ICLRRuntimeHost * CLRRuntimeInfo_get_clr_runtime(ICLRRuntimeInfo * info)
	{
		if (info != nullptr) {
			ICLRRuntimeHost* pHost = NULL;
			HRESULT hr = info->GetInterface(CLSID_CLRRuntimeHost, IID_ICLRRuntimeHost, (LPVOID*)&pHost);
			if (!FAILED(hr)) {
				return pHost;
			}
		}
		return NULL;
	}

	ICorRuntimeHost * CLRRuntimeInfo_get_cor_runtime(ICLRRuntimeInfo * info)
	{
		if (info != nullptr) {
			ICorRuntimeHost* pHost = NULL;
			HRESULT hr = info->GetInterface(CLSID_CorRuntimeHost, IID_ICorRuntimeHost, (LPVOID*)&pHost);
			if (!FAILED(hr)) {
				return pHost;
			}
		}
		return NULL;
	}

	_AppDomain * CorRuntimeHost_get_app_domain(ICorRuntimeHost * host)
	{
		if (host != nullptr) {
			IUnknown* spThunk = NULL;
			HRESULT hr = host->GetDefaultDomain(&spThunk);
			if (!FAILED(hr)) {
				_AppDomain* appDomain = NULL;
				hr = spThunk->QueryInterface(&appDomain);
				if (!FAILED(hr)) {
					return appDomain;
				}
			}
		}
		return NULL;
	}

	_Assembly * AppDomain_load_assembly(_AppDomain* appDomain, char * assemblyName)
	{
		if (appDomain != NULL) {
			_Assembly* assembly = NULL;
			int iBufferSize = MultiByteToWideChar(CP_UTF8, MB_COMPOSITE, assemblyName, -1, 0, 0);
			BSTR buffer = SysAllocStringLen(0, iBufferSize);
			MultiByteToWideChar(CP_UTF8, MB_COMPOSITE, assemblyName, -1, buffer, iBufferSize);
			HRESULT hr = appDomain->Load_2(buffer, &assembly);
			if (!FAILED(hr)) {
				return assembly;
			}
		}
		return NULL;
	}

}