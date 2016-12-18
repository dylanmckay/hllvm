use ir::Constant;

pub struct BlockAddress<'ctx>(Constant<'ctx>);
impl_upcast!(BlockAddress => Constant);

impl<'ctx> BlockAddress<'ctx>
{
}
