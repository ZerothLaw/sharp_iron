use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::{self, DirEntry, read_dir};
use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

extern crate serde_json;
use serde_json::Value;

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

fn devenv_command(solution_path: &Path, task: &str, project: &str) {
	//find devenv location via vswhere
	let program_files = std::env::var("ProgramFiles(x86)").unwrap();

	let p = Path::new(&program_files).join("Microsoft Visual Studio").join("Installer").join("vswhere.exe");
	let vswhere = Command::new(p).args(&["-format", "json", "-latest", "-legacy"]).output();
	match vswhere {
		Ok(output) => {
			match output.status.success() {
				true => {
					let stdout = String::from_utf8_lossy(&output.stdout);
					let devenv_path: Value  = match serde_json::from_str(&stdout) {
						Ok(v) => {
							v
						}
						Err(ex) => { panic!(ex); }
					};
					let devenv_path = &devenv_path[0];
					let devenv_path = &devenv_path["productPath"];
					println!("Running command... {} {:?}", devenv_path.to_string(), &[solution_path.to_str().unwrap(), task, project, "/Project", "clr_c_api"]);
					let vs_command = Command::new(devenv_path.to_string()).args(&[solution_path.to_str().unwrap(), task, project, "/Project", "clr_c_api"]).output();
					let vs_command = match vs_command {
						Ok(comm) => comm, 
						Err(ex) => {panic!("Command failed! {}", ex);}
					};
					println!("Build results: {:?}", String::from_utf8_lossy(&vs_command.stdout));
				}, 
				false => { panic!(output.status);}
			}
		},
		Err(ex) => {
			println!("Could not find vswhere or vswhere failed.");
			println!("error: {:?}", ex);
			panic!("vswhere failed.");
		}
	};
	println!("devenv_command");
}

fn build_c_lib( dir: &Path, extensions: Vec<&str>) -> bool{
	println!("build_c_lib: ({:?}, {:?})",  dir, extensions);
	let timestamp_file = dir.join(".timestamps.json");
	if do_build(timestamp_file, dir, extensions.clone()) {
		let proj_name = "static_debug";
		devenv_command(Path::new(".\\clr_c_api\\clr_c_api.sln"), "/Clean", proj_name);
		devenv_command(Path::new(".\\clr_c_api\\clr_c_api.sln"), "/Build", proj_name);

		let proj_name = "dylib_debug";
		devenv_command(Path::new(".\\clr_c_api\\clr_c_api.sln"), "/Clean", proj_name);
		devenv_command(Path::new(".\\clr_c_api\\clr_c_api.sln"), "/Build", proj_name);

		let timestamps = get_timestamps(dir, extensions);
		let timestamp_file = dir.join(".timestamps.json");
		let timestamp_file = fs::File::create(timestamp_file).unwrap();
		serde_json::to_writer(timestamp_file, &timestamps).unwrap();
		return true;
	}
	false
}

fn copy_command(fle: &PathBuf, dest: PathBuf, overwrite: bool) {
	let res = match Command::new("xcopy").args(&[fle.to_str().unwrap(), dest.to_str().unwrap(), {match overwrite{ true => "/Y", false => ""}}]).output() {
		Ok(res) => res, 
		Err(ex) => { println!("{:?} exception.", ex); panic!(ex);}
	};
	println!("Copy results: {:?}", &res.stdout);
}

fn get_rust_lib_home() -> PathBuf {
	env::home_dir().
		unwrap().
		join(".rustup").
		join("toolchains").
		join("stable-x86_64-pc-windows-msvc").
		join("lib").
		join("rustlib").
		join("x86_64-pc-windows-msvc").
		join("lib")
}

fn copy_c_lib() {
	let p = Path::new(".\\clr_c_api\\x64\\");
	let proj_name = "static_debug";
	let fle_name = "clr_c_api.lib";
	let p2 = p.join(proj_name).join(fle_name);
	copy_command(&p2, get_rust_lib_home(), true);
	copy_command(&p2, PathBuf::from("."), true);

	let proj_name = "dylib_debug";
	let fle_name = "clr_c_api.dll";
	let p2 = p.join(proj_name).join(fle_name);
	copy_command(&p2, get_rust_lib_home(), true);
	copy_command(&p2, PathBuf::from("."), true);
}


fn main() {
	let refresh = build_c_lib(Path::new(".\\clr_c_api\\clr_c_api\\"), vec!("cpp", "h", "def", "vcxproj"));
	if refresh {
		copy_c_lib();
	}
	println!("cargo:rustc-link-lib=static=clr_c_api");
	println!("cargo:rustc-link-search=static=.\\clr_c_api\\x64\\static_debug");
}