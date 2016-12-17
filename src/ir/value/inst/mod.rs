pub use self::terminator::*;

pub mod terminator;

use ir::Value;

pub struct Instruction<'ctx>(Value<'ctx>);

impl_upcast!(Instruction => Value);
