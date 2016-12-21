pub use self::fcmp::FCmpInst;
pub use self::icmp::ICmpInst;

pub mod fcmp;
pub mod icmp;

use ir::Instruction;

pub struct CmpInst<'ctx>(Instruction<'ctx>);
impl_upcast!(CmpInst => Instruction);
