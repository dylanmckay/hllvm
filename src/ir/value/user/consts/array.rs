use ir::Constant;

pub struct ConstantArray<'ctx>(Constant<'ctx>);
impl_subtype!(ConstantArray => Constant);

impl<'ctx> ConstantArray<'ctx>
{
}
