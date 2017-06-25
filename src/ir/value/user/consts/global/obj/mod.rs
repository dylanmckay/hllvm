pub use self::func::Function;
pub use self::variable::GlobalVariable;

pub mod func;
pub mod variable;

use ir::GlobalValue;

pub struct GlobalObject<'ctx>(GlobalValue<'ctx>);
impl_subtype!(GlobalObject => GlobalValue);
