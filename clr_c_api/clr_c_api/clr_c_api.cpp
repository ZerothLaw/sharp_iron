// clr_c_api.cpp : Defines the exported functions for the DLL application.
//

#include "stdafx.h"
#include "clr_c_api.h"
#include <mscoree.h>

#include <cstdlib>
#include <string>
#include <iostream>

extern "C" {
	IRustHostControl* RustHostControl_new() {
		RustHostControl* pHost = new RustHostControl();
		return pHost;
	}	
}

RustHostControl::RustHostControl()
{
	refCount_m = 0;
	defaultDomainManager_m = NULL;
}

RustHostControl::~RustHostControl()
{
	if (defaultDomainManager_m != NULL) {
		defaultDomainManager_m->Release();
	}
}

ICustomAppDomainManager * RustHostControl::GetDomainManager()
{
	if (defaultDomainManager_m) {
		defaultDomainManager_m->AddRef();
	}
	return defaultDomainManager_m;
}

HRESULT RustHostControl::QueryInterface(const IID & iid, void ** ppv)
{
	if (!ppv) return E_POINTER;
	*ppv = this;
	AddRef();
	return S_OK;
}

ULONG RustHostControl::AddRef()
{
	return InterlockedIncrement(&refCount_m);
}

ULONG RustHostControl::Release()
{
	if (InterlockedDecrement(&refCount_m) == 0) {
		delete this;
		return 0;
	}
	return refCount_m;
}

HRESULT RustHostControl::GetHostManager(REFIID id, void ** ppHostManager)
{
	*ppHostManager = NULL;
	return E_NOINTERFACE;
}

HRESULT RustHostControl::SetAppDomainManager(DWORD dwAppDomainID, IUnknown * pUnkAppDomainManager)
{
	HRESULT hr = S_OK;
	hr = pUnkAppDomainManager->QueryInterface(__uuidof(ICustomAppDomainManager), (PVOID*)&defaultDomainManager_m);
	return hr;
}
