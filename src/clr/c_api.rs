//c_api module

#![allow(dead_code)]
#![allow(non_snake_case)]
//std
use std::ffi::OsStr;

//3rd party
use winapi::Interface;
use winapi::shared::guiddef::{GUID};
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



pub struct ClrArray<T> {
	inner: T, 
	safe_array: Option<*mut SAFEARRAY>
}

impl<T> ClrArray<T> {
	pub fn new(input: T) -> ClrArray<T>{
		ClrArray{inner: input, safe_array: None}
	}

}

impl<T> ClrArray<T>
	where T: ToString
 {
	pub fn to_safearray(&self) -> Result<*mut SAFEARRAY, HRESULT> {
		let inner = self.inner.to_string();
		let mut sab: SAFEARRAYBOUND = SAFEARRAYBOUND {cElements: inner.len() as ULONG, lLbound: 0 as LONG};
		let psa = unsafe {SafeArrayCreate(VT_UI1 as u16, 1, &mut sab)};
		let hr = unsafe {
			SafeArrayLock(psa)
		};
		match hr {
			0 => {
				let data = unsafe {
					(*psa).pvData as *mut BYTE
				};
				let cin_str = inner.clone();
				let tdata: &[u8] = cin_str.as_bytes();
				let pdata: *const u8 = tdata.as_ptr();
				unsafe {
					pdata.copy_to(data, inner.len())
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

impl<T> ClrArray<T>
	where T:ClrWrapper 
{
	pub fn into_safearray(sz: usize) -> Result<*mut SAFEARRAY, HRESULT> {
		let mut sab: SAFEARRAYBOUND = SAFEARRAYBOUND{ cElements: sz as ULONG, lLbound: 0 as LONG};
		let psa = unsafe {
			SafeArrayCreate(T::vartype() as u16, 1, &mut sab)
		};
		if psa.is_null(){
			return Err(0);
		}
		Ok(psa)
	}

	// pub fn from_vec(sz: usize, arr: &[T]) ->Result<ClrArray<T>, HRESULT> {
	// 	let psa = match ClrArray::into_safearray(sz) {
	// 		Ok(psa) => psa, 
	// 		Err(hr) => return Err(hr)
	// 	};
	// 	let mut wrapper = ClrArray::new();
	// 	Ok(psa
	// }
}

pub trait ClrWrapper{
	type InnerPointer;

	fn into_raw(&self) -> *mut Self::InnerPointer;
	fn vartype() -> VARTYPE;
}

pub struct NetArray<W> 
	where W: ClrWrapper
{
	inner: Vec<W>, 
	safe_array: Option<*mut SAFEARRAY>,
	dirty: bool
} 

impl<W> NetArray<W> 
	where W: ClrWrapper
{
	pub fn from_vec<V: Into<Vec<W>>>(v: V) -> NetArray<W> {
		let v = v.into();
		NetArray {
			inner: v, 
			safe_array: None, 
			dirty: false
		}
	}

	pub fn into_safearray(&self, sz: usize) -> Result<*mut SAFEARRAY, HRESULT>{
		let mut sab: SAFEARRAYBOUND = SAFEARRAYBOUND{ cElements: sz as ULONG, lLbound: 0 as LONG};
		let psa = unsafe {
			SafeArrayCreate(W::vartype() as u16, 1, &mut sab)
		};
		if psa.is_null(){
			return Err(0);
		}
		Ok(psa)
	}

	pub fn to_safearray(&self) -> Result<*mut SAFEARRAY, HRESULT>
	{
		if !self.dirty && self.safe_array.is_some() {
			return match self.safe_array {
				Some(psa) => Ok(psa),
				None => Err(-1)
			};
		}
		let psa = match self.into_safearray(self.inner.len()) {
			Ok(psa) => psa,
			Err(hr) => return Err(hr)
		};

		let hr = unsafe {
			SafeArrayLock(psa)
		};
		match hr {
			0 => {
				let mut data = unsafe {
					(*psa).pvData as *mut *mut W::InnerPointer
				};

				for wrapper in &self.inner {
					unsafe {
						let raw = wrapper.into_raw();
						*data = raw;
						data = data.add(1);
					}
				}
				
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