// clr_c_api.cpp : Defines the exported functions for the DLL application.
//

#include "stdafx.h"
#include "clr_c_api.h"
#include <mscoree.h>
#import <mscorlib.tlb> rename("value","val") rename("or", "ORR") rename("ReportEvent", "InteropServices_ReportEvent") raw_interfaces_only no_smart_pointers

#include <cstdlib>
#include <string>
#include <iostream>


extern "C" {
	#define BUILD_RESULT(result, func_call, ptr) {\
		result.hr = func_call;\
		result.ok = !FAILED(result.hr);\
		result.c_ptr = result.ok ? (void *)ptr: NULL;\
	}

	CAPIResult newCLRMetaHost() {
		CAPIResult result;
		ICLRMetaHost *pMetaHost = NULL;
		BUILD_RESULT(result, CLRCreateInstance(CLSID_CLRMetaHost, IID_ICLRMetaHost, (LPVOID*)&pMetaHost), pMetaHost);
		return result;
	}

	CAPIResult CLRMetaHost_get_runtime(ICLRMetaHost* host, const char* version) {
		CAPIResult result;
		result.hr = 0;
		result.ok = false;
		if (host != NULL) {
			ICLRRuntimeInfo *pRuntimeInfo = NULL;
			int iLen = MultiByteToWideChar(CP_ACP, 0, version, -1, 0, 0);
			wchar_t* wc_version = SysAllocStringLen(0, iLen);
			MultiByteToWideChar(CP_ACP, 0, version, -1, wc_version, iLen);
			BUILD_RESULT(result, host->GetRuntime(wc_version, IID_ICLRRuntimeInfo, (LPVOID*)&pRuntimeInfo), pRuntimeInfo);
			SysFreeString(wc_version);
			return result;
		}
		else {
			result.ok = false;
			result.hr = E_HANDLE;
		}
		return result;
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

	CAPIResult CLRRuntimeInfo_get_clr_runtime(ICLRRuntimeInfo * info)
	{
		CAPIResult result;
		result.hr = E_HANDLE;
		result.ok = false;
		if (info != NULL) {
			ICLRRuntimeHost* pHost = NULL;
			BUILD_RESULT(result, info->GetInterface(CLSID_CLRRuntimeHost, IID_ICLRRuntimeHost, (LPVOID*)&pHost), pHost);
			return result;
		}
		return result;
	}

	bool CLRRuntimeHost_start(ICLRRuntimeHost * host)
	{
		SamplehostControl* hostControl = new SamplehostControl();
		HRESULT hr = host->SetHostControl((IHostControl*)hostControl);
		if (!FAILED(hr)) {
			hr = host->Start();
			return !FAILED(hr);
		}
		return false;
	}
	CAPIResult CLRRuntimeHost_load_assembly(ICLRRuntimeInfo * info, ICLRRuntimeHost * host, char * assemblyName)
	{
		ICorRuntimeHost* corHost = NULL;
		CAPIResult result;
		result.hr = E_HANDLE;
		result.ok = false;
		result.c_ptr = (void*)1;
		HRESULT hr = info->GetInterface(CLSID_CorRuntimeHost, IID_PPV_ARGS(&corHost));
		if (!FAILED(hr)) {
			result.c_ptr = (void*)2;
			IUnknown* appDomainThunk;
			hr = corHost->GetDefaultDomain(&appDomainThunk);
			if (!FAILED(hr)) {
				result.c_ptr = (void*)3;
				_AppDomain* appDomain = NULL;
				hr = appDomainThunk->QueryInterface(IID_PPV_ARGS(&appDomain));
				if (!FAILED(hr)) {
					result.c_ptr = (void*)4;
					_Assembly* assembly = NULL;
					long *pRetVal = NULL;
					int iLen = MultiByteToWideChar(CP_ACP, 0, assemblyName, -1, 0, 0);
					BSTR bs_assemblyName = SysAllocStringLen(0, iLen);
					MultiByteToWideChar(CP_ACP, 0, assemblyName, -1, bs_assemblyName, iLen);
					//printf("%S", bs_assemblyName);
					BUILD_RESULT(result, appDomain->Load_2(bs_assemblyName, &assembly), assembly);
					SysFreeString(bs_assemblyName);
					result.c_ptr = result.ok ? result.c_ptr : (void*)5;
					return result;	
				}
			}
		}
		result.hr = hr;
		return result;
	}
	CAPIResult CLRRuntimeHost_stop(ICLRRuntimeHost* host) {
		CAPIResult result;
		result.hr = E_HANDLE;
		result.ok = false;
		if (host != NULL) {
			BUILD_RESULT(result, host->Stop(), NULL);
		}
		return result;
	}

	CAPIResult Assembly_release(_Assembly* assembly) {
		CAPIResult result;
		result.hr = E_HANDLE;
		result.ok = false;
		if (assembly != NULL) {
			assembly->Release();
			result.hr = S_OK;
			result.ok = true;
		}
		return result;
	}
}

SamplehostControl::SamplehostControl()
{
	refCount_m = 0;
	defaultDomainManager_m = NULL;
}

SamplehostControl::~SamplehostControl()
{
	if (defaultDomainManager_m != NULL) {
		defaultDomainManager_m->Release();
	}
}

FacadeInterface * SamplehostControl::GetFacade()
{
	if (defaultDomainManager_m) {
		defaultDomainManager_m->AddRef();
	}
	return defaultDomainManager_m;
}

HRESULT SamplehostControl::QueryInterface(const IID & iid, void ** ppv)
{
	if (!ppv) return E_POINTER;
	*ppv = this;
	AddRef();
	return S_OK;
}

ULONG SamplehostControl::AddRef()
{
	return InterlockedIncrement(&refCount_m);
}

ULONG SamplehostControl::Release()
{
	if (InterlockedDecrement(&refCount_m) == 0) {
		delete this;
		return 0;
	}
	return refCount_m;
}

HRESULT SamplehostControl::GetHostManager(REFIID id, void ** ppHostManager)
{
	*ppHostManager = NULL;
	return E_NOINTERFACE;
}

HRESULT SamplehostControl::SetAppDomainManager(DWORD dwAppDomainID, IUnknown * pUnkAppDomainManager)
{
	HRESULT hr = S_OK;
	hr = pUnkAppDomainManager->QueryInterface(__uuidof(FacadeInterface), (PVOID*)&defaultDomainManager_m);
	return hr;
}
