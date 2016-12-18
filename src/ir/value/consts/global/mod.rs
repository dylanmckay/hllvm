pub use self::obj::*;

pub mod obj;

use ir::Constant;

pub struct GlobalValue<'ctx>(Constant<'ctx>);
impl_upcast!(GlobalValue => Constant);
