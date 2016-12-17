use {ValueRef, TypeRef, ModuleRef};
use libc;

extern "C" {
    pub fn LLVMRustFunctionCreate(ty: TypeRef,
                                  linkage: libc::c_uint,
                                  name: *const libc::c_char,
                                  module: ModuleRef) -> ValueRef;
}
