use ir::Constant;

pub struct ConstantFP<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantFP => Constant);

impl<'ctx> ConstantFP<'ctx>
{
}
