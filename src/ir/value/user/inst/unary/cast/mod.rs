pub use self::addrspacecast::AddrSpaceCastInst;
pub use self::fptrunc::FPTruncInst;
pub use self::inttoptr::IntToPtrInst;
pub use self::ptrtoint::PtrToIntInst;
pub use self::sext::SExtInst;
pub use self::trunc::TruncInst;
pub use self::zext::ZExtInst;

pub mod addrspacecast;
pub mod fptrunc;
pub mod inttoptr;
pub mod ptrtoint;
pub mod sext;
pub mod trunc;
pub mod zext;

use ir::UnaryInst;

pub struct CastInst<'ctx>(UnaryInst<'ctx>);
impl_upcast!(CastInst => UnaryInst);
