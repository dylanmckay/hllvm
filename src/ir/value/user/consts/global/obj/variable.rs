use SafeWrapper;
use ir::{GlobalObject, GlobalValue, Value, Type, Linkage, ThreadLocalMode, Constant, User};
use sys;

use std::ptr;

/// A global variable.
pub struct GlobalVariable<'ctx>(GlobalObject<'ctx>);

impl<'ctx> GlobalVariable<'ctx>
{
    /// Creates a new global constant.
    pub fn new_constant(value: &Value,
                        linkage: Linkage,
                        tls_mode: ThreadLocalMode,
                        address_space: u32) -> Self {
        GlobalVariable::new_generic(&value.ty(), true, linkage,
            Some(value), tls_mode, address_space, false)
    }

    /// Creates a new global variable.
    pub fn new_generic(ty: &Type,
                       is_constant: bool,
                       linkage: Linkage,
                       initializer: Option<&Value>,
                       tls_mode: ThreadLocalMode,
                       address_space: u32,
                       is_externally_initialized: bool) -> Self {
        let initializer = initializer.map(SafeWrapper::inner).unwrap_or(ptr::null_mut());

        unsafe {
            let inner = sys::LLVMRustCreateGlobalVariable(ty.inner(), is_constant,
                linkage, initializer, tls_mode, address_space as _, is_externally_initialized);
            GlobalVariable(GlobalObject(GlobalValue(Constant(User(Value::from_inner(inner))))))
        }
    }
}

impl_subtype!(GlobalVariable => GlobalObject);
