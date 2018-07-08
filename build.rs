use std::path::{Path};
use std::process::Command;

extern crate cc;

fn set_vs_env<'a>(cmd: &'a mut Command, devenv_dir: &Path) -> &'a mut Command {
	cmd
		.env("DevEnvDir", devenv_dir)
		.env("Framework40Version", "v4.0")
		.env("FrameworkDir", "C:\\Windows\\Microsoft.NET\\Framework\\")
		.env("FrameworkDir32", "C:\\Windows\\Microsoft.NET\\Framework\\")
		.env("FrameworkVersion", "v4.0.30319")
		.env("FrameworkVersion32", "v4.0.30319")
		.env("LIBPATH", "C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30319")
		.env("Path", "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\MSBuild\\15.0\\bin;C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30319;C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\IDE\\;C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\Tools\\;%PATH%")
		.env("VisualStudioVersion", "15.0")
		.env("VS150COMNTOOLS", "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\Common7\\Tools\\")
		.env("VSCMD_ARG_app_plat", "Desktop")
		.env("VSCMD_ARG_HOST_ARCH", "x86")
		.env("VSCMD_ARG_no_ext", "1")
		.env("VSCMD_ARG_TGT_ARCH", "x86")
		.env("VSCMD_ARG_winsdk", "none")
		.env("VSCMD_VER", "15.7.3")
		.env("VSINSTALLDIR", "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\")
		.env("__DOTNET_ADD_32BIT", "1")
		.env("__DOTNET_PREFERRED_BITNESS", "32")
}

fn build_and_move_csharp_lib() {

	match Command::new("C:\\Program Files (x86)\\Microsoft Visual Studio\\Installer\\vswhere.exe")
		.args(&["-format", "value", "-property", "productPath", "-latest", "-legacy"])
		.output() 
	{
			Ok(out) => {
				let devenv_path = String::from_utf8_lossy(&out.stdout);
				let devenv_path = Path::new(&*devenv_path);
				let devenv_dir = devenv_path.parent().unwrap();
				let msbuild_cmd = set_vs_env(&mut Command::new("msbuild"), devenv_dir)
					.args(&[".\\clr_c_api\\RustAppDomainManager\\RustAppDomainManager.csproj"])
					.output();
				match msbuild_cmd {
					Ok(msbuild_out) => {
						println!("{:?}", String::from_utf8_lossy(&msbuild_out.stdout));
						//now register
						let regasm = set_vs_env(&mut Command::new("regasm"), devenv_dir)
							.args(&["/tlb", ".\\clr_c_api\\RustAppDomainManager\\bin\\Debug\\RustAppDomainManager.dll"])
							.output();

						if regasm.is_ok() {
							let regasm = regasm.unwrap();
							println!("{}", String::from_utf8_lossy(&regasm.stdout));
							println!("{}", String::from_utf8_lossy(&regasm.stderr));
						}

						let xcopy = Command::new("xcopy")
										.args(&[".\\clr_c_api\\RustAppDomainManager\\bin\\Debug\\RustAppDomainManager.tlb", ".\\clr_c_api\\clr_c_api\\", "/Y"])
										.output();

						if xcopy.is_ok() {
							println!("{}", String::from_utf8_lossy(&xcopy.unwrap().stdout));
						}


					},
					Err(ex) => panic!(ex)
				}
			},
		Err(ex) => panic!(ex)
	}
}

fn build_c_lib() {
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

fn main() {
	build_and_move_csharp_lib();
	build_c_lib();
}