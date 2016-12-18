use ir::Value;

pub struct Constant<'ctx>(Value<'ctx>);
impl_upcast!(Constant => Value);
