use SafeWrapper;
use ir::{Context, FunctionType, Attribute, Value, Function};
use Upcast;

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

    pub fn get_or_insert_function(&self,
                                  name: &str,
                                  func_ty: &FunctionType,
                                  attributes: &[Attribute]) -> Function {
        let name = ffi::CString::new(name).unwrap();
        let attrs: Vec<_> = attributes.iter().map(Attribute::inner).collect();

        unsafe {
            let func = sys::LLVMRustModuleGetOrInsertFunction(self.inner,
                                                   name.as_ptr(),
                                                   func_ty.upcast_ref().inner(),
                                                   &attrs);
            Function::from_value(Value::from_inner(func))
        }
    }

    /// Dumps the module to standard error.
    pub fn dump(&self) {
        unsafe { sys::LLVMRustModuleDump(self.inner) }
    }
}

impl<'ctx> SafeWrapper for Module<'ctx>
{
    type Inner = sys::ModuleRef;

    unsafe fn from_inner(inner: sys::ModuleRef) -> Self {
        Module { inner: inner, phantom: marker::PhantomData }
    }

    fn inner(&self) -> sys::ModuleRef { self.inner }
}
