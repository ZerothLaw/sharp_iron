fn main() {
	println!("cargo:rustc-link-lib=static=clr_c_api");
	println!("cargo:rustc-link-search=static=C:\\Users\\coolt\\source\\repos\\sharp_iron\\clr_c_api\\x64\\Debug");
}