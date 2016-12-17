use ir::{Context, Value};
use sys;

use std::ptr;

pub struct ReturnInst<'ctx>(Value<'ctx>);

impl<'ctx> ReturnInst<'ctx>
{
    pub fn new(ret_val: Option<&Value>,
               context: &Context) -> Self {
        let ret_val = ret_val.map(Value::inner).unwrap_or(ptr::null_mut());

        let val = unsafe { sys::LLVMRustCreateReturnInst(context.inner(), ret_val) };
        ReturnInst(Value::new(val))
    }
}

impl_upcast!(ReturnInst => Value);
