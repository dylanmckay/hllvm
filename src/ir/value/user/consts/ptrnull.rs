use ir::Constant;

pub struct ConstantPointerNull<'ctx>(Constant<'ctx>);
impl_upcast!(ConstantPointerNull => Constant);

impl<'ctx> ConstantPointerNull<'ctx>
{
}
