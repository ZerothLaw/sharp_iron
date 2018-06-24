//mod.rs
//core C ABI 
mod c_api;

//public types 
mod metahost;
mod runtime_info;
mod runtime_host;
mod assembly;
mod type_;

//make types public
//ordering is in layers of references.
pub use self::metahost::MetaHost;
pub use self::runtime_info::RuntimeInfo;
pub use self::runtime_host::RuntimeHost;
pub use self::assembly::Assembly;
pub use self::type_::Type_;