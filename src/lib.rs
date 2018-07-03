/*
	Design comments. 
	 Wrap CLR Unmanaged Hosting API classes/structs with FFI vtables (like winapi)
	 then wrap them with a safe interface
*/

extern crate libc;
#[macro_use]
extern crate winapi;
extern crate widestring;

#[allow(dead_code)]//, dead_codenon_snake_case)]
mod clr;
pub use clr::MetaHost;
pub use clr::RuntimeInfo;
pub use clr::RuntimeHost;

#[cfg(test)]
mod tests {
	use super::*;
	
	// #[test]
	// fn metahost() { 
	// 	let host = MetaHost::new();
	// 	assert_eq!(host.is_null(), false);
	// }
	
	// #[test]
	// fn runtime_info() {
	// 	let host = MetaHost::new();
	// 	let mut runtime_info = host.get_runtime_info("v4.0.30319");
	// 	assert_eq!(runtime_info.is_null(), false);
	// 	assert_eq!(runtime_info.is_loadable(), true);
	// }
	
	// #[test]
	// fn get_clr_host() {
	// 	let host = MetaHost::new();
	// 	let runtime_info = host.get_runtime_info("v4.0.30319");
	// 	let clr_host = match runtime_info.get_clr_host() {
	// 		Ok(new_host) => new_host, 
	// 		Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
	// 	};
	// 	assert_eq!(clr_host.is_null(), false);
	// }
	
	// #[test]
	// fn start_runtime() { 
	// 	let host = MetaHost::new();
	// 	let runtime_info = host.get_runtime_info("v4.0.30319");
	// 	let mut clr_host = match runtime_info.get_clr_host() {
	// 		Ok(new_host) => new_host, 
	// 		Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
	// 	};
	// 	assert_eq!(clr_host.start(), true);
	// }

	#[test]
	fn get_app_domain() { 
		use std::env;
		println!("CWD = {:?}", env::current_dir());
		let host = MetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let mut clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		assert_eq!(clr_host.start(), true);
		let host = match clr_host.get_host_control() {
			Some(hst) => hst, 
			None => {
				panic!("get_host_control call failed");
			}
		};
		let domain_manager = host.get_domain_manager();
		let val = domain_manager.load_assembly("System, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089, processorArchitecture=MSIL");
		println!("load_assembly exit");
		println!("{:?}", val);

		let oval = domain_manager.load_assembly("TestAssembly, Version=1.0.0.0, Culture=neutral, PublicKeyToken=c97610437c81cba6, processorArchitecture=MSIL");
		println!("{:?}", oval);
	}
}
