use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self, DirEntry, read_dir};
use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

extern crate serde_json;

fn get_files(dir: &Path) -> Vec<DirEntry> {
	if dir.is_dir() {
		return match read_dir(dir) {
			Ok(reader) => {
				reader.map(|x| x.unwrap()) .collect()
			}, 
			_ => {Vec::new()}
		};
	}
	Vec::new()
}

fn get_timestamps(dir: &Path, extensions: Vec<&str>) -> HashMap<PathBuf, SystemTime> {
	let mut timestamps = HashMap::new();
	let files = get_files(dir);
	for file in files {
		let filepath = file.path();
		if filepath.is_file() {
			let ext = &filepath.extension().unwrap();
			for extension in &extensions {
				if &OsString::from(extension) == ext {
					timestamps.insert(filepath.clone(), file.metadata().unwrap().modified().unwrap());
				}
			}
		}
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
				if new_timestamps.get(key).unwrap() != original_timestamps.get(key).unwrap() {
					return true;
				}
			}
		}
		return false;
	}
	
	true
}


fn build_c_lib( dir: &Path, extensions: Vec<&str>) -> bool{
	println!("build_c_lib: ({:?}, {:?})",  dir, extensions);
	let timestamp_file = dir.join(".timestamps.json");
	if do_build(timestamp_file, dir, extensions.clone()) {
		println!("Calling build_c_lib.bat .\\clr_c_api\\clr_c_api.sln, %USERPROFILE%\\.rustup, stable, 64");
		let _bat_command = Command::new("call").args(&["build_c_lib.bat", ".\\clr_c_api\\clr_c_api.sln", "%USERPROFILE%\\.rustup", "stable", "64"]).output();

		let timestamps = get_timestamps(dir, extensions);
		let timestamp_file = dir.join(".timestamps.json");
		let timestamp_file = fs::File::create(timestamp_file).unwrap();
		serde_json::to_writer(timestamp_file, &timestamps).unwrap();
		return true;
	}
	false
}

fn main() {
	let _refresh = build_c_lib(Path::new(".\\clr_c_api\\clr_c_api\\"), vec!("cpp", "h", "def", "vcxproj"));
	
	println!("cargo:rustc-link-lib=static=clr_c_api");
	println!("cargo:rustc-link-search=static=.\\clr_c_api\\x64\\static_debug");
}