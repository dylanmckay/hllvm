use ir::Constant;

pub struct UndefValue<'ctx>(Constant<'ctx>);
impl_upcast!(UndefValue => Constant);

impl<'ctx> UndefValue<'ctx>
{
}
