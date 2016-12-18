use ir::Constant;

pub struct ConstantVector<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantVector => Constant);

impl<'ctx> ConstantVector<'ctx>
{
}
