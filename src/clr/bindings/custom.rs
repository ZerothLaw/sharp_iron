
#![allow(dead_code)]
#![allow(non_snake_case)]

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, };

use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR};

use clr::bindings::IHostControl;
use clr::bindings::IHostControlVtbl;

use clr::bindings::_AppDomain;

RIDL!{#[uuid(0xB47320A6, 0x6265, 0x4C34, 0x90, 0xAC, 0x3F, 0xF2, 0xA9, 0x09, 0x68, 0x6C)]
interface ICustomAppDomainManager(ICustomAppDomainManagerVtbl): IDispatch(IDispatchVtbl){
    fn get_app_domain(
        friendly_name: BSTR,
        app_domain: *mut *mut _AppDomain,
    ) -> HRESULT,
    fn create_app_domain(
        name: BSTR,
        app_domain: *mut *mut _AppDomain,
    )-> HRESULT,

}}

RIDL!{#[uuid(0x1e20d486, 0x67c7, 0x4cd6, 0xb5, 0x6b, 0x41, 0xd2, 0x29, 0x7d, 0x5b, 0x2f)]
interface IRustHostControl(IRustHostControlVtbl) : IHostControl(IHostControlVtbl){
    fn  domain_manager() -> *mut ICustomAppDomainManager,
}}