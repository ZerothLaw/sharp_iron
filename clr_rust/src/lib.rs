extern crate libc;
extern crate winapi;
use winapi::ctypes::{c_void};

#[link(name="clr_c_api", kind="static")]
extern "C" {
	fn newCLRMetaHost() -> *mut c_void;
}

#[no_mangle]
pub fn wrapper() -> *mut c_void {
	unsafe {
		newCLRMetaHost()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn new_clr() {
		let clr:*mut c_void = wrapper();
		assert_eq!(!clr.is_null(), true);
	}

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
