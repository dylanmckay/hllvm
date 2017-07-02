//! The module-related code.

use {SafeWrapper, Subtype};
use ir::{Context, FunctionType, Attribute, Value, Function, GlobalVariable};

use sys;
use std::marker;

use std::ffi;

/// A group of functions and globals representing a single compilation unit.
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

    pub fn get_or_insert_function(&mut self,
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

    /// Adds a global variable to the module.
    pub fn add_global(&mut self, global: &GlobalVariable) {
        unsafe { sys::LLVMRustModuleAddGlobal(self.inner(), global.inner()) }
    }

    /// Removes a global variable from the module.
    pub fn remove_global(&mut self, global: &GlobalVariable) {
        unsafe { sys::LLVMRustModuleRemoveGlobal(self.inner(), global.inner()) }
    }

    /// Adds a function to the module.
    pub fn add_function(&mut self, function: &Function) {
        unsafe { sys::LLVMRustModuleAddFunction(self.inner(), function.inner()) }
    }

    /// Removes a function from the module.
    pub fn remove_function(&mut self, function: &Function) {
        unsafe { sys::LLVMRustModuleRemoveFunction(self.inner(), function.inner()) }
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
