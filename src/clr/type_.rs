//type_.rs
//std

//3rd party
use winapi::ctypes::{c_long};
use winapi::shared::minwindef::{ULONG};
use winapi::shared::winerror::{HRESULT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

//self
RIDL!{#[uuid(0xbca8b44d, 0xaad6, 0x3a86, 0x8a, 0xb7, 0x03, 0x34, 0x9f, 0x4f, 0x2d, 0xa2)]
interface _Type(_TypeVtbl): IUnknown(IUnknownVtbl){
	fn GetTypeInfoCount(
		pcTInfo: *mut ULONG,
	) -> HRESULT,
	fn GetTypeInfo(
		iTInfo: ULONG,
		lcid: ULONG,
		ppTInfo: c_long,
	) -> HRESULT,
}}

/*    virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Guid (
        /*[out,retval]*/ GUID * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Module (
        /*[out,retval]*/ struct _Module * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Assembly (
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_TypeHandle (
        /*[out,retval]*/ struct RuntimeTypeHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FullName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Namespace (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_AssemblyQualifiedName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetArrayRank (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_BaseType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructors (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterface (
        /*[in]*/ BSTR name,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterfaces (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall FindInterfaces (
        /*[in]*/ struct _TypeFilter * filter,
        /*[in]*/ VARIANT filterCriteria,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvent (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _EventInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvents (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvents_2 (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedTypes (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedType (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember (
        /*[in]*/ BSTR name,
        /*[in]*/ enum MemberTypes Type,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetDefaultMembers (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall FindMembers (
        /*[in]*/ enum MemberTypes MemberType,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _MemberFilter * filter,
        /*[in]*/ VARIANT filterCriteria,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetElementType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsSubclassOf (
        /*[in]*/ struct _Type * c,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall IsInstanceOfType (
        /*[in]*/ VARIANT o,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall IsAssignableFrom (
        /*[in]*/ struct _Type * c,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterfaceMap (
        /*[in]*/ struct _Type * interfaceType,
        /*[out,retval]*/ struct InterfaceMapping * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethods (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetField (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFields (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperties (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMembers (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * namedParameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_UnderlyingSystemType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructor (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ enum CallingConventions callConvention,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructor_2 (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructor_3 (
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructors_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_TypeInitializer (
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ enum CallingConventions callConvention,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_4 (
        /*[in]*/ BSTR name,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_5 (
        /*[in]*/ BSTR name,
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_6 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethods_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetField_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFields_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterface_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvent_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _EventInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_4 (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_5 (
        /*[in]*/ BSTR name,
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_6 (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Type * returnType,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_7 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperties_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedTypes_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedType_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember_3 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMembers_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum TypeAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNotPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedFamANDAssem (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedFamORAssem (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAutoLayout (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsLayoutSequential (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsExplicitLayout (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsInterface (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsValueType (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSealed (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsEnum (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsImport (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSerializable (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAnsiClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsUnicodeClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAutoClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsArray (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsByRef (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPointer (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrimitive (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsCOMObject (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_HasElementType (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsContextful (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsMarshalByRef (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals_2 (
        /*[in]*/ struct _Type * o,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
*/