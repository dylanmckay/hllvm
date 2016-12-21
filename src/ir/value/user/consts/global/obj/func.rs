use SafeWrapper;
use ir::{Value, Constant, FunctionType, Module, GlobalValue, GlobalObject, Linkage, User,
         Block};
use sys;

use std::ffi;

/// A function.
pub struct Function<'ctx>(GlobalObject<'ctx>);

impl<'ctx> Function<'ctx>
{
    /// Creates a new function.
    pub fn new(ty: &FunctionType,
               linkage: Linkage,
               name: &str,
               module: &Module) -> Self {
        let name = ffi::CString::new(name).unwrap();

        unsafe {
            let inner = sys::LLVMRustFunctionCreate(ty.inner(),
                            linkage, name.as_ptr(), module.inner());
            Function::from_value(Value::from_inner(inner))
        }
    }

    pub fn append(&mut self, block: &mut Block) {
        unsafe { sys::LLVMRustFunctionAddBlock(self.inner(), block.inner()) }
    }

    pub unsafe fn from_value(val: Value<'ctx>) -> Self {
        Function(GlobalObject(GlobalValue(Constant(User(val)))))
    }
}

impl_upcast!(Function => GlobalObject);
