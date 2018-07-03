//c_api module

#![allow(dead_code)]
#![allow(non_snake_case)]
//std

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