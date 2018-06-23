use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::{self, DirEntry, read_dir};
use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

extern crate serde_json;

fn get_files(dir: &Path, extensions: Vec<&str>) -> Vec<DirEntry> {
	if dir.is_dir() {
		return match read_dir(dir) {
			Ok(reader) => {
				reader.map(|x| x.unwrap()).filter(|x| {
					let path = x.path();
					if path.is_file() {
						if let Some(ext) = path.extension() {
							return extensions.iter().any(|extension| &OsString::from(extension) == ext);
						}
					}
					
					return false;
					
				}).collect()
			}, 
			_ => {Vec::new()}
		};
	}
	Vec::new()
}

fn get_timestamps(dir: &Path, extensions: Vec<&str>) -> HashMap<PathBuf, SystemTime> {
	let mut timestamps = HashMap::new();
	let files = get_files(dir, extensions);
	for file in files {
		let filepath = file.path();
		timestamps.insert(filepath.clone(), file.metadata().unwrap().modified().unwrap());
	}
	timestamps
}

fn do_build(timestamp_path: PathBuf, dir: &Path, extensions: Vec<&str>) -> bool {
	if timestamp_path.exists() {
		let file = fs::File::open(timestamp_path).unwrap();
		let original_timestamps: HashMap<PathBuf, SystemTime> = serde_json::from_reader(file).unwrap();
		let new_timestamps = get_timestamps(dir, extensions);
		for key in original_timestamps.keys() {
			if new_timestamps.contains_key(key) {
				let new_sys_time = new_timestamps.get(key).unwrap();
				let old_sys_time = original_timestamps.get(key).unwrap();
				if  new_sys_time > old_sys_time {
					return true;
				}
			}
		}
		return true;
	}
	true
}


fn build_c_lib( dir: &Path, extensions: Vec<&str>) -> bool{
	println!("build_c_lib: ({:?}, {:?})",  dir, extensions);
	let out_dir = env::var("OUT_DIR").unwrap();
	let path_dir = Path::new(&out_dir);
	let timestamp_file = path_dir.join("timestamps.json");
	if do_build(timestamp_file, dir, extensions.clone()) {
		println!("Calling build_c_lib.bat stable 64 .\\clr_c_api\\x64\\ {} Debug", out_dir);
		let _bat_command = Command::new("build_c_lib.bat").args(&["stable", "64", ".\\clr_c_api\\", &out_dir, "Debug"]).output().expect("build_c_lib.bat call failed!");
		let timestamps = get_timestamps(dir, extensions);
		let timestamp_file = path_dir.join("timestamps.json");
		let timestamp_file = fs::File::create(timestamp_file).unwrap();
		serde_json::to_writer(timestamp_file, &timestamps).unwrap();
		return true;
	}
	false
}

fn main() {
	let _refresh = build_c_lib(Path::new(".\\clr_c_api\\clr_c_api\\"), vec!("cpp", "h", "def", "vcxproj"));
	let out_dir = env::var("OUT_DIR").unwrap();
	println!("cargo:rustc-link-lib=clr_c_api");
	println!("cargo:rustc-link-search={}", &out_dir);
}