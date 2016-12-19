use {ContextRef, AttributeRef};
use libc;

cpp! {
    #include "llvm/IR/Attributes.h"

    pub fn LLVMRustAttributeGetInteger(ctx: ContextRef as "llvm::LLVMContext*",
                                       kind: libc::c_uint as "unsigned",
                                       val: u64 as "uint64_t") -> AttributeRef as "llvm::Attribute*" {
        llvm::Attribute *attr = new llvm::Attribute();
        *attr = llvm::Attribute::get(*ctx, (llvm::Attribute::AttrKind)kind, val);
        return attr;
    }

    pub fn LLVMRustAttributeGetString(ctx: ContextRef as "llvm::LLVMContext*",
                                      kind: *const libc::c_char as "const char*",
                                      val: *const libc::c_char as "const char*") -> AttributeRef as "llvm::Attribute*" {
        llvm::Attribute *attr = new llvm::Attribute();
        *attr = llvm::Attribute::get(*ctx, kind, val);
        return attr;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_support::Context;

    use std::ffi::CString;

    #[test]
    fn can_get_integer_attribute() {
        let context = Context::new();

        unsafe {
            LLVMRustAttributeGetInteger(context.inner, 0, 50);
        }
    }

    #[test]
    fn can_get_string_attribute() {
        let context = Context::new();

        let kind = CString::new("hello").unwrap();
        let value = CString::new("world").unwrap();

        unsafe {
            LLVMRustAttributeGetString(context.inner, kind.as_ptr(), value.as_ptr());
        }
    }
}
