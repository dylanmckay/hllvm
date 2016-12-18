pub use self::inst::*;
pub use self::consts::*;

pub mod inst;
pub mod consts;

use ir::Value;

pub struct User<'ctx>(Value<'ctx>);
impl_upcast!(User => Value);
