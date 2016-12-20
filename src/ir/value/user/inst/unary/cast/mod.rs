pub use self::addrspacecast::AddrSpaceCastInst;
pub use self::inttoptr::IntToPtrInst;
pub use self::ptrtoint::PtrToIntInst;

pub mod addrspacecast;
pub mod inttoptr;
pub mod ptrtoint;

use ir::UnaryInst;

pub struct CastInst<'ctx>(UnaryInst<'ctx>);
impl_upcast!(CastInst => UnaryInst);
