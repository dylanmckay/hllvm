pub use self::addrspacecast::AddrSpaceCastInst;
pub mod addrspacecast;

use ir::UnaryInst;

pub struct CastInst<'ctx>(UnaryInst<'ctx>);
impl_upcast!(CastInst => UnaryInst);
