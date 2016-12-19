pub use self::consts::*;
pub mod consts;

use {ValueRef, TypeRef};

cpp! {
    #include "llvm/IR/Value.h"

    pub fn LLVMRustValueDump(value: ValueRef as "llvm::Value*") {
        value->dump();
    }

    pub fn LLVMRustValueGetType(value: ValueRef as "llvm::Value*")
        -> TypeRef as "llvm::Type*" {
        return value->getType();
    }
}

#[cfg(test)]
mod test {
    use test_support::Context;

    #[test]
    fn can_get_type_from_constant() {
        let ctx = Context::new();

        unsafe {
            let val = ::LLVMRustConstantIntGetTrue(ctx.inner);
            ::LLVMRustValueGetType(val);
        }
    }
}
