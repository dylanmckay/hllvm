use ir::Constant;

pub struct ConstantAggregateZero<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantAggregateZero => Constant);

impl<'ctx> ConstantAggregateZero<'ctx>
{
}
