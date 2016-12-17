use TypeRef;

pub enum OpaqueValue { }
pub type ValueRef = *mut OpaqueValue;

extern "C" {
    pub fn LLVMRustValueDump(_: ValueRef);
    pub fn LLVMRustValueGetType(_: ValueRef) -> TypeRef;
}

#[cfg(test)]
mod test {
    #[test]
    fn can_get_type_from_constant() {
        unsafe {
            let ctx = ::LLVMRustCreateContext();
            let val = ::LLVMRustConstantIntGetTrue(ctx);
            ::LLVMRustValueGetType(val);

            ::LLVMRustDestroyContext(ctx);
        }
    }
}
