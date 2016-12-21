use SafeWrapper;
use ir::{User, Instruction, Value, TerminatorInst, Block};
use sys;

use std::ptr;

/// A conditional or unconditional branch.
pub struct BranchInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> BranchInst<'ctx>
{
    /// Creates an unconditional branch.
    pub fn unconditional(destination: &Block) -> Self {
        unsafe { BranchInst::new(None, destination, None) }
    }

    /// Creates a conditional branch.
    ///
    /// At least one of `on_true` or `on_false` must be specified.
    pub fn conditional(condition: &Value,
                       on_true: &Block,
                       on_false: Option<&Block>) -> Self {
        unsafe { BranchInst::new(Some(condition), on_true, on_false) }
    }

    /// Creates a new branch.
    unsafe fn new(condition: Option<&Value>,
                  on_true: &Block,
                  on_false: Option<&Block>) -> Self {
        let condition = condition.map(Value::inner).unwrap_or(ptr::null_mut());
        let on_false = on_false.map(|b| b.inner()).unwrap_or(ptr::null_mut());

        let inner = sys::LLVMRustCreateBranchInst(on_true.inner(), on_false, condition);
        BranchInst(TerminatorInst(Instruction(User(Value::from_inner(inner)))))
    }
}

impl_upcast!(BranchInst => TerminatorInst);

#[cfg(test)]
mod test
{
    use ir;

    #[test]
    fn can_create_unconditional() {
        let context = ir::Context::new();
        let bb = ir::Block::new(&context);
        ir::BranchInst::unconditional(&bb);
    }

    #[test]
    fn can_create_conditional() {
        let context = ir::Context::new();
        let bb = ir::Block::new(&context);
        let t = ir::ConstantInt::boolean_true(&context);
        ir::BranchInst::conditional(&t, &bb, None);
    }

    #[test]
    fn can_create_conditional_else() {
        let context = ir::Context::new();
        let bbtrue = ir::Block::new(&context);
        let bbfalse = ir::Block::new(&context);

        let t = ir::ConstantInt::boolean_true(&context);
        ir::BranchInst::conditional(&t, &bbtrue, Some(&bbfalse));
    }
}
