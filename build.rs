use std::path::{Path};

extern crate cc;

fn main() {
	let c_dir = Path::new(".\\clr_c_api\\clr_c_api\\");
	let c_dir = c_dir.to_path_buf();
	let windows_include = Path::new("C:\\Program Files (x86)\\Windows Kits\\NETFXSDK\\4.6.1\\Include\\um");
	let mscoree_include = Path::new("C:\\Program Files (x86)\\Windows Kits\\NETFXSDK\\4.6.1\\Lib\\um\\x64");
	let clr_runtime_include = Path::new("C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30319");
	cc::Build::new()
		.cpp(true)
		.files(&[c_dir.join("clr_c_api.cpp"), c_dir.join("dllmain.cpp"), c_dir.join("stdafx.cpp")])
		.include(windows_include)
		.include(clr_runtime_include)
		.include(mscoree_include)
		.flag(&format!("/DEF:{:?}", c_dir.join("Source.def")))
		.compile("clr_api");
	
	println!("cargo:rustc-link-search={}", &mscoree_include.to_str().unwrap());
}