use winapi::Interface;
use winapi::shared::guiddef::{GUID};

#[repr(C)]
pub enum PrincipalPolicy
{
    UnauthenticatedPrincipal = 0,
    NoPrincipal = 1,
    WindowsPrincipal = 2
}
add_uuid!(PrincipalPolicy, 0x7d29bc4b, 0x8fbc, 0x38aa, 0x8b, 0x35, 0xed, 0x45, 0x39, 0xa1, 0xcf, 0x8e);

#[repr(C)]
pub enum AssemblyBuilderAccess
{
    Run = 1,
    Save = 2,
    RunAndSave = 3,
    ReflectionOnly = 6,
    RunAndCollect = 9
}
add_uuid!(AssemblyBuilderAccess, 0xf0778630, 0xac34, 0x3d71, 0x9f, 0xab, 0x61, 0x7f, 0x61, 0x24, 0x30, 0x65);