//mod.rs
//core C ABI 
#[macro_use]
mod c_api;

//public types 
mod metahost;
mod runtime_info;
mod runtime_host;
mod assembly;
mod type_;
mod method;
mod misc;
mod field;
mod host_control;
mod app_domain;
mod object_handle;

//make types public
//ordering is in layers of references.
pub use self::metahost::MetaHost;
pub use self::runtime_info::RuntimeInfo;
pub use self::runtime_host::RuntimeHost;
pub use self::host_control::{RustDomainManager, RustHostControl};
pub use self::app_domain::AppDomain;
pub use self::assembly::Assembly;
