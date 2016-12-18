pub use self::func::Function;

pub mod func;

use ir::Constant;

pub struct GlobalValue<'ctx>(Constant<'ctx>);
impl_upcast!(GlobalValue => Constant);
