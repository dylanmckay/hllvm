use ir::{Value, Context};
use sys;

use std::{ptr, ffi};

/// A basic block.
pub struct Block<'ctx>
{
    pub ty: Value<'ctx>,
}

impl<'ctx> Block<'ctx>
{
    pub fn new(context: &Context,
               name: Option<&str>,
               // FIXME: make this `Function`
               parent: Option<&Value>,
               insert_before: Option<&Self>) -> Self {
        let insert_before = insert_before.map_or(ptr::null_mut(), |b| b.ty.inner());

        let name = ffi::CString::new(name.unwrap_or("")).unwrap();

        let ty = unsafe {
            sys::LLVMRustBasicBlockCreate(context.inner(),
                                          name.as_ptr(),
                                          parent.map_or(ptr::null_mut(), Value::inner),
                                          insert_before)
        };

        Block { ty: Value::new(ty) }
    }

    pub fn upcast_ref(&self) -> &Value<'ctx> { &self.ty }
    pub fn upcast(self) -> Value<'ctx> { self.ty }
}

impl<'a> AsRef<Value<'a>> for Block<'a>
{
    fn as_ref(&self) -> &Value<'a> { &self.ty }
}
