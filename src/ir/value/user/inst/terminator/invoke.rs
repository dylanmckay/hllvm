use SafeWrapper;
use ir::{User, Function, Instruction, Value, TerminatorInst, Block};
use sys;

/// A invoke instruction.
pub struct InvokeInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> InvokeInst<'ctx>
{
    /// Creates a new invoke instruction.
    pub fn new(func: &Function,
               args: &[&Value],
               on_success: &Block,
               on_error: &Block) -> Self {
        let args: Vec<_> = args.iter().map(|v| v.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustCreateInvokeInst(func.inner(), &args,
                on_success.inner(), on_error.inner());
            wrap_value!(inner => User => Instruction => TerminatorInst => InvokeInst)
        }
    }
}

impl_subtype!(InvokeInst => TerminatorInst);
