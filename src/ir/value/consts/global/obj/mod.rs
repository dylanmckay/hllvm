pub use self::func::Function;

pub mod func;

use ir::GlobalValue;

pub struct GlobalObject<'ctx>(GlobalValue<'ctx>);
impl_upcast!(GlobalObject => GlobalValue);
