pub use self::terminator::*;

pub mod terminator;

use ir::User;

pub struct Instruction<'ctx>(User<'ctx>);
impl_upcast!(Instruction => User);
