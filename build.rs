use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

extern crate serde_json;

fn get_files(dir: Path) -> Vec<DirEntry> {
	if dir.is_dir() {
		match fs::read_dir(dir) {
			Ok(reader) => reader.collect()
		}
	}
	Vec::new()
}

fn get_timestamps(dir: Path, extensions: Vec<&str>) -> HashMap<Path, SystemTime> {
	
}

fn do_build(timestamp_path: Path, dir: Path, extensions: Vec<&str>) -> bool {
	if timestamp_path.exists() {
		//compare timestamps
		let file = fs::File::open(timestamp_path).unwrap();
		let original_timestamps: HashMap<Path, SystemTime> = serde_json::from_reader(file);
		let new_timestamps = get_timestamps(dir, extensions);
		
		
	}
	//otherwise, write timestamps to file, then return true
	let timestamps = get_timestamps(dir, extensions);
	let mut timestamp_file = fs::File::open(timestamp_path).unwrap();
	serde_json::to_writer(timestamp_file, &timestamps);
	true
}


fn build_lib() {
	let _res = Command::new("C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\IDE\\devenv.exe").args(&[".\\clr_c_api\\clr_c_api.sln", "/Clean", "static_debug"]).output().expect("aaaaaaa");
	let res = Command::new("C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\IDE\\devenv.exe").args(&[".\\clr_c_api\\clr_c_api.sln", "/Build", "static_debug"]).output().expect("whoops");
	println!("Build results: {:?}", res.stdout);
}

fn build_dll() {
	let _res = Command::new("C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\IDE\\devenv.exe").args(&[".\\clr_c_api\\clr_c_api.sln", "/Clean", "dylib_debug"]).output().expect("aaaaaaa");
	let res = Command::new("C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\IDE\\devenv.exe").args(&[".\\clr_c_api\\clr_c_api.sln", "/Build", "dylib_debug"]).output().expect("whoops");
	println!("Build results: {:?}", res.stdout);
}

fn copy_lib() {

	let p = Path::new(".\\clr_c_api\\x64\\static_debug\\clr_c_api.lib");

	let p2 = env::home_dir().unwrap().join(".rustup").join("toolchains").join("stable-x86_64-pc-windows-msvc").join("lib").join("rustlib").join("x86_64-pc-windows-msvc").join("lib");
	let res = match Command::new("xcopy").args(&[p.to_str().unwrap(), p2.to_str().unwrap(), "/Y"]).output() {
		Ok(res) => res, 
		Err(ex) => { println!("{:?} exception.", ex); panic!(ex);}
	};
	println!("Copy results: {:?}", res.stdout);
	
	let _res2 = Command::new("xcopy").args(&[p.to_str().unwrap(), ".", "/Y"]).output().unwrap();
}

fn copy_dll() {

	let p = Path::new(".\\clr_c_api\\x64\\dylib_debug\\clr_c_api.dll");

	let p2 = env::home_dir().unwrap().join(".rustup").join("toolchains").join("stable-x86_64-pc-windows-msvc").join("lib").join("rustlib").join("x86_64-pc-windows-msvc").join("lib");
	let res = match Command::new("xcopy").args(&[p.to_str().unwrap(), p2.to_str().unwrap(), "/Y"]).output() {
		Ok(res) => res, 
		Err(ex) => { println!("{:?} exception.", ex); panic!(ex);}
	};
	println!("Copy results: {:?}", res.stdout);
	let _res2 = Command::new("xcopy").args(&[p.to_str().unwrap(), ".", "/Y"]).output().unwrap();
}

fn main() {
	build_lib();
	copy_lib();
	build_dll();
	copy_dll();
	println!("cargo:rustc-link-lib=static=clr_c_api");
	println!("cargo:rustc-link-search=static=.\\clr_c_api\\x64\\static_debug");
}