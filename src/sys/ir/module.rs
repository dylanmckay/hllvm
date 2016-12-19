use {ContextRef, ValueRef, TypeRef, AttributeRef};
use libc;

pub enum OpaqueModule { }
pub type ModuleRef = *mut OpaqueModule;

cpp! {
    #include "llvm/IR/Module.h"

    pub fn LLVMRustCreateModule(id: *const libc::c_char as "const char*",
                                context: ContextRef as "llvm::LLVMContext*")
        -> ModuleRef as "llvm::Module*" {
        return new llvm::Module(llvm::StringRef(id), *context);
    }

    // TODO: make this use the array struct
    pub fn LLVMRustModuleGetOrInsertFunction(module: ModuleRef as "llvm::Module*",
                                             name: *const libc::c_char as "const char*",
                                             func_ty: TypeRef as "llvm::Type*",
                                             attributes: *mut AttributeRef as "llvm::Attribute**",
                                             attr_count: libc::size_t as "size_t")
        -> ValueRef as "llvm::Value*" {
        llvm::FunctionType *FnTy = llvm::dyn_cast<llvm::FunctionType>(func_ty);
        assert(FnTy && "must be a function type");

        llvm::AttributeSet AttrSet;

        for (size_t i=0; i<attr_count; ++i) {
          AttrSet.addAttribute(FnTy->getContext(), i, *attributes[i]);
        }

        return module->getOrInsertFunction(name, FnTy, AttrSet);
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
