use {ContextRef, ValueRef, TypeRef, AttributeRef};
use libc;

pub enum OpaqueModule { }
pub type ModuleRef = *mut OpaqueModule;

cpp! {
    #include "ffi_helpers.h"

    #include "llvm/IR/Module.h"

    pub fn LLVMRustCreateModule(id: *const libc::c_char as "const char*",
                                context: ContextRef as "llvm::LLVMContext*")
        -> ModuleRef as "llvm::Module*" {
        return new llvm::Module(llvm::StringRef(id), *context);
    }

    pub fn LLVMRustModuleGetOrInsertFunction(module: ModuleRef as "llvm::Module*",
                                             name: *const libc::c_char as "const char*",
                                             func_ty: TypeRef as "llvm::Type*",
                                             attributes: &[AttributeRef] as "support::Slice<llvm::Attribute*>")
        -> ValueRef as "llvm::Value*" {
        llvm::AttributeSet AttrSet;

        for (size_t i=0; i<attributes.len(); ++i) {
          AttrSet.addAttribute(func_ty->getContext(), i, *attributes[i]);
        }

        return module->getOrInsertFunction(name, support::cast<llvm::FunctionType>(func_ty), AttrSet);
    }

    pub fn LLVMRustModuleAddGlobal(module: ModuleRef as "llvm::Module*",
                                   global: ValueRef as "llvm::Value*") {
        module->getGlobalList().addNodeToList(support::cast<llvm::GlobalVariable>(global));
    }

    pub fn LLVMRustModuleRemoveGlobal(module: ModuleRef as "llvm::Module*",
                                      global: ValueRef as "llvm::Value*") {
        module->getGlobalList().removeNodeFromList(support::cast<llvm::GlobalVariable>(global));
    }

    pub fn LLVMRustModuleAddFunction(module: ModuleRef as "llvm::Module*",
                                     func: ValueRef as "llvm::Value*") {
        module->getFunctionList().addNodeToList(support::cast<llvm::Function>(func));
    }

    pub fn LLVMRustModuleRemoveFunction(module: ModuleRef as "llvm::Module*",
                                        func: ValueRef as "llvm::Value*") {
        module->getFunctionList().removeNodeFromList(support::cast<llvm::Function>(func));
    }

    pub fn LLVMRustModuleDump(module: ModuleRef as "llvm::Module*") {
        module->dump();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_support::Context;

    use std::ffi::CString;

    #[test]
    fn can_create_module() {
        let ctx = Context::new();
        let module_name = CString::new("hello").unwrap();

        unsafe {
            LLVMRustCreateModule(module_name.as_ptr(), ctx.inner);
        }
    }
}
