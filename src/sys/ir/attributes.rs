use {ContextRef, AttributeRef};
use libc;

extern "C" {
  pub fn LLVMRustAttributeGetInteger(_: ContextRef,
                                     kind: libc::c_uint,
                                     val: u64) -> AttributeRef;

  pub fn LLVMRustAttributeGetString(_: ContextRef,
                                    kind: *const libc::c_char,
                                    val: *const libc::c_char) -> AttributeRef;
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
