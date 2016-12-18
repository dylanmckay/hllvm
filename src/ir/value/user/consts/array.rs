use ir::Constant;

pub struct ConstantArray<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantArray => Constant);

impl<'ctx> ConstantArray<'ctx>
{
}
