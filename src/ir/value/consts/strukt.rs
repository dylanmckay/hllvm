use ir::Constant;

pub struct ConstantStruct<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantStruct => Constant);

impl<'ctx> ConstantStruct<'ctx>
{
}
