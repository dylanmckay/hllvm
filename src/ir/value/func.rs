use ir::{Value, FunctionType, Module};
use sys;

use std::ffi;

/// A function.
pub struct Function<'ctx>(Value<'ctx>);

impl<'ctx> Function<'ctx>
{
    pub fn new(ty: &FunctionType,
               linkage: u64,
               name: &str,
               module: &Module) -> Self {
        let name = ffi::CString::new(name).unwrap();

        let val = unsafe {
            sys::LLVMRustFunctionCreate(ty.upcast_ref().inner(),
                                        linkage as _,
                                        name.as_ptr(),
                                        module.inner())
        };

        Function(Value::new(val))
    }

    pub fn from_value(val: Value<'ctx>) -> Self {
        Function(val)
    }

    pub fn upcast_ref(&self) -> &Value<'ctx> { &self.0 }
    pub fn upcast(self) -> Value<'ctx> { self.0 }
}

impl<'a> AsRef<Value<'a>> for Function<'a>
{
    fn as_ref(&self) -> &Value<'a> { &self.0 }
}
