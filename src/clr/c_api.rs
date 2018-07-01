//c_api module
//std

//3rd party
use winapi::shared::guiddef::{REFIID, REFCLSID};
use winapi::shared::minwindef::{LPVOID};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::ntdef::HANDLE;

//self
use clr::host_control::IRustHostControl;

extern "C" {
	pub fn CLRCreateInstance(
		clsid: REFCLSID, 
		riid: REFIID, 
		ppInterface: *mut LPVOID,
	) -> HRESULT;

	pub fn RustHostControl_new() -> *mut IRustHostControl;
	pub fn GetCurrentProcess() -> HANDLE;
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