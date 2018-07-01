//c_api module
//std

//3rd party
use winapi::shared::guiddef::{REFIID, REFCLSID};
use winapi::shared::minwindef::{LPVOID};
use winapi::shared::winerror::{HRESULT};
use winapi::shared::ntdef::HANDLE;

//self
use clr::host_control::IRustHostControl;
use clr::host_control::ICustomAppDomainManager;
use clr::misc::_AppDomain;

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