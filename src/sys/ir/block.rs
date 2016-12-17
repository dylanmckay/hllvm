use {ContextRef, ValueRef};
use libc;

extern "C" {
    pub fn LLVMRustBasicBlockCreate(_: ContextRef,
                                    name: *const libc::c_char,
                                    func: ValueRef,
                                    insert_before: ValueRef) -> ValueRef;
}
