//object handle that wraps objects returned from app domains
//std

//3rd party

//self
use clr::bindings::_ObjectHandle;

//body

pub struct ObjectHandle {
    ptr: *mut _ObjectHandle,
}
