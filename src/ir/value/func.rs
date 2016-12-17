use ir::{Value, FunctionType, Module};
use Upcast;
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
}

impl_upcast!(Function => Value);
