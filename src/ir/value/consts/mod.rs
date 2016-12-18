pub use self::int::ConstantInt;

pub mod int;

use ir::Value;

pub struct Constant<'ctx>(Value<'ctx>);
impl_upcast!(Constant => Value);
