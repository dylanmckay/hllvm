use ContextRef;

cpp! {
    #include "llvm/IR/LLVMContext.h"

    pub fn LLVMRustCreateContext() -> ContextRef as "llvm::LLVMContext*" {
        return new llvm::LLVMContext();
    }

    pub fn LLVMRustDestroyContext(ctx: ContextRef as "llvm::LLVMContext*") {
        delete ctx;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_and_destroy() {
        let ctx = unsafe { LLVMRustCreateContext() };
        unsafe { LLVMRustDestroyContext(ctx) };
    }
}
