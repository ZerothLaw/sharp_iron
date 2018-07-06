//c_api module

#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ffi::OsStr;

//3rd party
use winapi::um::oaidl::{SAFEARRAY, SAFEARRAYBOUND};
use winapi::shared::guiddef::{REFIID, REFCLSID};
use winapi::shared::minwindef::{LPVOID, BYTE};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::ntdef::ULONG;
use winapi::shared::ntdef::LONG;
use winapi::shared::wtypes::{VT_UI1, VARTYPE};
use winapi::shared::minwindef::UINT;
use winapi::um::oleauto::{SysAllocStringLen};

//self
use clr::host_control::{IRustHostControl};

extern "C" {
	pub fn CLRCreateInstance(
		clsid: REFCLSID, 
		riid: REFIID, 
		ppInterface: *mut LPVOID,
	) -> HRESULT;

	pub fn RustHostControl_new() -> *mut IRustHostControl;
	pub fn GetCurrentProcess() -> HANDLE;
	pub fn SafeArrayCreate(vt: VARTYPE, cDims: UINT, rgsabound: *mut SAFEARRAYBOUND) -> *mut SAFEARRAY;
	pub fn SafeArrayDestroy(safe: *mut SAFEARRAY)->HRESULT;
	pub fn SafeArrayLock(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayUnlock(psa: *mut SAFEARRAY) -> HRESULT;
}

pub struct ClrArray {
	inner: String
}

impl ClrArray {
	pub fn new(input: &str) -> ClrArray{
		ClrArray{inner: String::from(input)}
	}

	pub fn to_safearray(&self) -> Result<*mut SAFEARRAY, HRESULT> {
		let mut sab: SAFEARRAYBOUND = SAFEARRAYBOUND {cElements: self.inner.len() as ULONG, lLbound: 0 as LONG};
		let psa = unsafe {SafeArrayCreate(VT_UI1 as u16, 1, &mut sab)};
		let hr = unsafe {
			SafeArrayLock(psa)
		};
		match hr {
			0 => {
				let data = unsafe {
					(*psa).pvData as *mut BYTE
				};
				let cin_str = self.inner.clone();
				let tdata: &[u8] = cin_str.as_bytes();
				let pdata: *const u8 = tdata.as_ptr();
				unsafe {
					pdata.copy_to(data, self.inner.len())
				};
				unsafe {
					SafeArrayUnlock(psa)
				};
				Ok(psa)
			}, 
			_ => {
				unsafe {
					SafeArrayDestroy(psa)
				};
				Err(hr)
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BString {
	size: usize, 
	inner: Vec<u16>
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BStr {
	size: usize,
	inner: [u16]
}

use std::ffi::{OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

pub fn os_to_wide(s: &OsStr) -> Vec<u16> {
	s.encode_wide().collect()
}

pub fn os_from_wide(s: &[u16]) -> OsString {
	OsString::from_wide(s)
}

impl BString {
	pub fn new() -> BString {
		BString {inner: vec![], size: 0}
	}
	pub fn from_vec<T>(raw: T) -> BString 
		where T: Into<Vec<u16>>
	{
		let raw = raw.into();
		let l = raw.len();
		BString { inner: raw, size: l }
	}

	pub fn from_str<S>(s: &S) -> BString 
		where S: AsRef<OsStr> + ?Sized 
	{
		let s = os_to_wide(s.as_ref());
		let l = s.len();
		BString {
			inner: s, 
			size: l
		}
	}

	pub fn into_vec(self) -> Vec<u16> {
		self.inner
	}

	pub unsafe fn as_sys(&self) -> *mut u16 {
		let new_ws = self.inner.clone();
		let ws_ptr = new_ws.as_ptr();
		SysAllocStringLen(ws_ptr, self.size as UINT)
	}
}

impl Into<Vec<u16>> for BString {
	fn into(self) -> Vec<u16> {
		self.into_vec()
	}
}

impl From<String> for BString {
	fn from(s: String) -> BString {
		BString::from_str(&s)
	}
}

#[macro_export]
macro_rules! coerce_pointer {
	//ident, [star][mutability] [type], ident2
	//=> let [mutability] ident2: [star][mutability] [type] = [ref][mutability] _ as [star][mutability] [type];
	( $pointer:ident, *mut $ct:ty, $ptr2:ident) => {
		let $ptr2: *mut $ct = &mut $pointer as *mut _ as *mut $ct;
	};
	( $pointer:ident, *const $ct:ty, $pt2:ident) => {
		let $ptr2: *const $ct = &$pointer as *const _ as *const $ct;
	};
}
#[macro_export]
macro_rules! add_uuid {
	($interface:ident, $lng:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr ) => {
		impl Interface for $interface {
			#[inline]
			fn uuidof() -> GUID {
				GUID {
					Data1: $lng,
					Data2: $w1, 
					Data3: $w2, 
					Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8]
				}
			}
		}
	};
}