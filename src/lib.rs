extern crate libc;
extern crate winapi;
extern crate widestring;

mod clr;
use clr::MetaHost;

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn metahost() { 
		let host = MetaHost::new();
		assert_eq!(host.is_null(), false);
	}
	
	#[test]
	fn runtime_info() {
		let host = MetaHost::new();
		let mut runtime_info = host.get_runtime_info("v4.0.30319");
		assert_eq!(runtime_info.is_null(), false);
		assert_eq!(runtime_info.is_loadable(), true);
	}
	
	#[test]
	fn get_clr_host() {
		let host = MetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		assert_eq!(clr_host.is_null(), false);
	}
	
	#[test]
	fn start_runtime() { 
		let host = MetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let mut clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		assert_eq!(clr_host.start(), true);
	}
	//TestAssembly, Version=1.0.0.0, Culture=neutral, PublicKeyToken=c97610437c81cba6
	//mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089
	#[test]
	fn load_assembly() {
		let host = MetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let mut clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => {/*println!("load_assembly c");*/ new_host}, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		
		let loaded_assembly = clr_host.load_assembly(runtime_info, "mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089");
		assert_eq!(loaded_assembly, true);
		
	}
}
