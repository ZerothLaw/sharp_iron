
#![allow(dead_code)]
#![allow(non_snake_case)]
//std

//3rd party
use winapi::Interface;
use winapi::ctypes::{c_long, c_short, c_void};
use winapi::shared::guiddef::{GUID, REFIID};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{LPCWSTR};

//self
use clr::bindings::_FieldInfo;
use clr::bindings::_MethodBase;
use clr::bindings::_PropertyInfo;
use clr::bindings::_Type;

//body





