use SafeWrapper;
use ir::{User, Instruction, Value, Function};
use sys;

/// A instruction which calls a fuction.
pub struct CallInst<'ctx>(Instruction<'ctx>);

impl<'ctx> CallInst<'ctx>
{
    /// Creates a new call instruction.
    pub fn new(func: &Function,
               args: &[&Value]) -> Self {
        let args: Vec<_> = args.iter().map(|v| v.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustCreateCallInst(func.inner(), &args);
            wrap_value!(inner => User => Instruction => CallInst)
        }
    }
}

impl_subtype!(CallInst => Instruction);
