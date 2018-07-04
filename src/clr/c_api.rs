//c_api module

#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ffi::OsStr;
use std::mem::size_of;

//3rd party
use widestring::{WideStr, WideCString, NulError, MissingNulError};

use winapi::ctypes::{c_void, wchar_t};
use winapi::um::oaidl::{SAFEARRAY, SAFEARRAYBOUND};
use winapi::shared::guiddef::{REFIID, REFCLSID};
use winapi::shared::minwindef::{LPVOID, BYTE};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::ntdef::ULONG;
use winapi::shared::ntdef::LONG;
use winapi::shared::wtypes::{VT_UI1, VARTYPE};
use winapi::shared::minwindef::UINT;
use winapi::um::combaseapi::{CoTaskMemAlloc, CoTaskMemFree};

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
pub struct BStr {
	contents_size: usize, 
	inner: WideCString
}

macro_rules! handle_iws {
	( $ws:ident ) => {
		match $ws {
			Ok(iws) => {
				let count = iws.len();
				Ok(BStr{ contents_size: count, inner: iws})
			}, 
			Err(et) => Err(et)
		}
	};
}

impl BStr {
	pub fn new<T>(v: T) -> Result<BStr, NulError> 
		where T: Into<Vec<u16>>
	{
		let ws = WideCString::new(v);
		handle_iws!(ws)
	}

	pub fn from_vec_with_nul<T>(v: T) -> Result<BStr, MissingNulError> 
		where T: Into<Vec<u16>>
	{
		let ws = WideCString::from_vec_with_nul(v);
		handle_iws!(ws)
	}

	pub fn from_str<T>(s: T) -> Result<BStr, NulError> 
		where T: AsRef<OsStr>
	{
		let ws = WideCString::from_str(s);
		handle_iws!(ws)
	}

	pub fn from_wide_str<T>(s: T) -> Result<BStr, NulError> 
		where T: AsRef<WideStr>
	{
		let ws = WideCString::from_wide_str(s);
		handle_iws!(ws)
	}

	pub fn from_wide_str_with_nul<T>(s: T) -> Result<BStr, NulError> 
		where T: AsRef<WideStr>
	{
		let ws = WideCString::from_wide_str(s);
		handle_iws!(ws)
	}

	//consumes self and the inner string
	pub unsafe fn as_raw(self) -> *mut u16 {
		let raw = self.inner.into_raw();
		let u16_sz = size_of::<u16>();
		let byte_step = 4 / u16_sz;
		let byte_sz = u16_sz * self.contents_size;
		
		//total size = <length prefix> + contents + <null ending bytes>
		let total_sz = 4 + byte_sz + size_of::<wchar_t>();
		let com_alloc_raw: *mut u16 = CoTaskMemAlloc(total_sz) as *mut u16;

		//write byte_sz to starting 4 bytes, but we need to lose upper half of usize (on 64 bit machines)
		let byte_sz_ptr: *const u16 = &byte_sz as *const _ as *const u16;
		//let byte_sz_ptr = byte_sz_ptr.add( byte_step);

		byte_sz_ptr.copy_to(com_alloc_raw, byte_step);
		let com_alloc_raw = com_alloc_raw.add(byte_step);
		
		raw.copy_to(com_alloc_raw, byte_sz);
		com_alloc_raw.add(byte_sz).write_bytes(0, 2);
		let _ws = WideCString::from_raw(raw); //grab string pointer up and ensure its cleared by Rust
		com_alloc_raw
	}

	//ONLY USE THIS ON THE POINTER YOU GET FROM as_raw !!! BAD STUFF CAN HAPPEN OTHERWISE
	pub unsafe fn free_raw(raw: *mut u16) {
		//first need to move back a byte_step
		let u16_sz = size_of::<u16>();
		let byte_step = 4 / u16_sz;

		let raw = raw.sub(byte_step);
		CoTaskMemFree(raw as *mut c_void);
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