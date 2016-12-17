use ir::Context;

use sys;
use std::marker;

use std::ffi;

/// An LLVM module.
/// `'ctx` - The lifetime of the context.
pub struct Module<'ctx>
{
    inner: sys::ModuleRef,
    phantom: marker::PhantomData<&'ctx ()>,
}

impl<'ctx> Module<'ctx>
{
    /// Creates a new module.
    pub fn new(id: &str, context: &'ctx Context) -> Self {
        let id = ffi::CString::new(id).unwrap();

        Module {
            inner: unsafe { sys::LLVMRustCreateModule(id.as_ptr(), context.inner()) },
            phantom: marker::PhantomData,
        }
    }

    /// Dumps the module to standard error.
    pub fn dump(&self) {
        unsafe { sys::LLVMRustModuleDump(self.inner) }
    }
}
