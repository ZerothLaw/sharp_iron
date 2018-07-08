use winapi::Interface;
use winapi::shared::guiddef::{GUID};

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

#[repr(C)]
pub enum MethodImplAttributes {
    CodeTypeMask = 3,
    IL = 0,
    Native = 1,
    OPTIL = 2,
    Runtime = -3,
    ManagedMask = 4,
    Unmanaged = -4,
    Managed = -1,
    ForwardRef = 16,
    PreserveSig = 128,
    InternalCall = 4096,
    Synchronized = 32,
    NoInlining = 8,
    NoOptimization = 64,
    MaxMethodImplVal = 65535
}
add_uuid!(MethodImplAttributes, 0xbcab3a5d, 0xf2cd, 0x3c69,0x84, 0x1d, 0xad, 0x00, 0x19, 0x69, 0xbf, 0x50);

#[repr(C)]
pub enum MemberTypes {
    Constructor = 1,
    Event = 2,
    Field = 4,
    Method = 8,
    Property = 16,
    TypeInfo = 32,
    Custom = 64,
    NestedType = 128,
    All = 191
}
add_uuid!(MemberTypes, 0x513b8b77, 0x4930, 0x36ba, 0x9a, 0x22, 0x0d, 0xae, 0xb2, 0x93, 0xe1, 0x09);

#[repr(C)]
pub enum PrincipalPolicy
{
    UnauthenticatedPrincipal = 0,
    NoPrincipal = 1,
    WindowsPrincipal = 2
}
add_uuid!(PrincipalPolicy, 0x7d29bc4b, 0x8fbc, 0x38aa, 0x8b, 0x35, 0xed, 0x45, 0x39, 0xa1, 0xcf, 0x8e);


#[repr(C)]
pub enum PropertyAttributes
{
    None = 0,
    SpecialName = 512,
    ReservedMask = 62464,
    RTSpecialName = 1024,
    HasDefault = 4096,
    Reserved2 = 8192,
    Reserved3 = 16384,
    Reserved4 = 32768
}
add_uuid!(PropertyAttributes, 0x816c979c, 0xd3d2, 0x3101, 0xb5, 0xca, 0xe4, 0xa5, 0xc5, 0xe9, 0x66, 0xfa);


#[repr(C)]
pub enum StreamingContextStates
{
    CrossProcess = 1,
    CrossMachine = 2,
    File = 4,
    Persistence = 8,
    Remoting = 16,
    Other = 32,
    Clone_ = 64,
    CrossAppDomain = 128,
    All = 255
}
add_uuid!(StreamingContextStates, 0x78304e50, 0xa1e6, 0x3d84, 0xa7, 0x18, 0x49, 0x02, 0x06, 0x81, 0xe0, 0x2e);


#[repr(C)]
pub enum BindingFlags
{
    Default_ = 0,
    IgnoreCase = 1,
    DeclaredOnly = 2,
    Instance = 4,
    Static = 8,
    Public = 16,
    NonPublic = 32,
    FlattenHierarchy = 64,
    InvokeMethod = 256,
    CreateInstance = 512,
    GetField = 1024,
    SetField = 2048,
    GetProperty = 4096,
    SetProperty = 8192,
    PutDispProperty = 16384,
    PutRefDispProperty = 32768,
    ExactBinding = 65536,
    SuppressChangeType = 131072,
    OptionalParamBinding = 262144,
    IgnoreReturn = 16777216
}
add_uuid!(BindingFlags, 0x3223e024, 0x5d70, 0x3236, 0xa9, 0x2a, 0x6b, 0x41, 0x14, 0xb2, 0x63, 0x2f);

#[repr(C)]
pub enum FieldAttributes
{
    FieldAccessMask = 7,
    PrivateScope = 0,
    Private = 1,
    FamANDAssem = 2,
    Assembly = 3,
    Family = 4,
    FamORAssem = 5,
    Public = 6,
    Static = 16,
    InitOnly = 32,
    Literal = 64,
    NotSerialized = 128,
    SpecialName = 512,
    PinvokeImpl = 8192,
    ReservedMask = 38144,
    RTSpecialName = 1024,
    HasFieldMarshal = 4096,
    HasDefault = 32768,
    HasFieldRVA = 256
}
add_uuid!(FieldAttributes, 0xc8679e0a, 0x1c67, 0x3a20, 0x86, 0x45, 0x0d, 0x93, 0x0f, 0x52, 0x90, 0x31);

#[repr(C)]
pub enum CallingConventions
{
    Standard = 1,
    VarArgs = 2,
    Any = 3,
    HasThis = 32,
    ExplicitThis = 64
}
add_uuid!(CallingConventions, 0xfd67ebe2, 0x30de, 0x3fbe, 0x89, 0x6b, 0x81, 0xda, 0x2e, 0x45, 0x51, 0x37);

#[repr(C)]
pub enum TypeAttributes
{
    VisibilityMask = 7,
    NotPublic = 0,
    Public = 1,
    NestedPublic = 2,
    NestedPrivate = 3,
    NestedFamily = 4,
    NestedAssembly = 5,
    NestedFamANDAssem = 6,
    NestedFamORAssem = -7,
    LayoutMask = 24,
    AutoLayout,
    SequentialLayout = 8,
    ExplicitLayout = 16,
    ClassSemanticsMask = 32,
    Class,
    Interface = -32,
    Abstract = 128,
    Sealed = 256,
    SpecialName = 1024,
    Import = 4096,
    Serializable = 8192,
    StringFormatMask = 196608,
    AnsiClass,
    UnicodeClass = 65536,
    AutoClass = 131072,
    CustomFormatClass = -196608,
    CustomFormatMask = 12582912,
    BeforeFieldInit = 1048576,
    ReservedMask = 264192,
    RTSpecialName = 2048,
    HasSecurity = 262144
}
add_uuid!(TypeAttributes, 0x28ee6224, 0xfd72, 0x3bdf, 0xb2, 0x48, 0xba, 0x91, 0x02, 0xfc, 0xeb, 0x14);