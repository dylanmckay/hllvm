pub use self::alloca::AllocaInst;
pub use self::load::LoadInst;
pub use self::extract_value::ExtractValueInst;
pub use self::cast::*;

pub mod alloca;
pub mod load;
pub mod extract_value;
pub mod cast;

use ir::Instruction;

pub struct UnaryInst<'ctx>(Instruction<'ctx>);
impl_upcast!(UnaryInst => Instruction);
