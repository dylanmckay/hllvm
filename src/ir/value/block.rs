use ir::{Value, Context, Function};
use sys;

use std::{ptr, ffi};

/// A basic block.
pub struct Block<'ctx>(Value<'ctx>);

impl<'ctx> Block<'ctx>
{
    pub fn new(context: &Context,
               name: Option<&str>,
               parent: Option<&Function>,
               insert_before: Option<&Self>) -> Self {
        let insert_before = insert_before.map_or(ptr::null_mut(), |b| b.0.inner());

        let name = ffi::CString::new(name.unwrap_or("")).unwrap();

        let ty = unsafe {
            sys::LLVMRustBasicBlockCreate(context.inner(),
                                          name.as_ptr(),
                                          parent.map_or(ptr::null_mut(), |f| f.upcast_ref().inner()),
                                          insert_before)
        };

        Block(Value::new(ty))
    }

    pub fn upcast_ref(&self) -> &Value<'ctx> { &self.0 }
    pub fn upcast(self) -> Value<'ctx> { self.0 }
}

impl<'a> AsRef<Value<'a>> for Block<'a>
{
    fn as_ref(&self) -> &Value<'a> { &self.0 }
}
