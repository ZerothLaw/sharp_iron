///std

//3rd party 

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

// self

RIDL!{#[uuid(0xaf93163f, 0xc2f4, 0x3fab, 0x9f, 0xf1, 0x72, 0x8a, 0x7a, 0xaa, 0xd1, 0xcb)]
interface _CrossAppDomainDelegate(_CrossAppDomainDelegateVtbl) : IDispatch(IDispatchVtbl){

}}
