//! The backend target code.

pub use self::registry::Registry;
pub use self::machine::Machine;
pub use self::compilation::Compilation;

// Reexports.
pub use sys::target::FileType;

pub mod registry;
pub mod machine;
pub mod compilation;

use SafeWrapper;
use sys;

use std::{ffi, fmt};

/// A target triple.
pub struct Triple(pub String);

/// A CPU name.
pub struct CPU(pub String);

/// An LLVM target.
pub struct Target(sys::TargetRef);

impl Target
{
    /// Gets the name of the target.
    pub fn name(&self) -> String {
        unsafe {
            let ptr = sys::LLVMRustTargetGetName(self.0);
            ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned()
        }
    }

    /// Gets the short description of the target.
    pub fn short_description(&self) -> String {
        unsafe {
            let ptr = sys::LLVMRustTargetGetShortDescription(self.0);
            ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned()
        }
    }

    /// Creates a machine object.
    pub fn create_machine(&self,
                          triple: &str,
                          cpu: &str,
                          features: &str) -> Machine {
        let triple = ffi::CString::new(triple).unwrap();
        let cpu = ffi::CString::new(cpu).unwrap();
        let features = ffi::CString::new(features).unwrap();

        unsafe {
            let inner = sys::LLVMRustTargetCreateTargetMachine(self.0,
                triple.as_ptr(), cpu.as_ptr(), features.as_ptr());
            Machine::from_inner(inner)
        }
    }

    /// Checks if the target has defined an `MCAsmBackend`.
    pub fn has_mc_asm_backend(&self) -> bool {
        unsafe { sys::LLVMRustTargetHasMCAsmBackend(self.0) }
    }

    /// Checks if the target has defined a `TargetMachine`.
    pub fn has_target_machine(&self) -> bool {
        unsafe { sys::LLVMRustTargetHasTargetMachine(self.0) }
    }

    /// Checks if the target supports just-in-time compilation.
    pub fn has_jit(&self) -> bool {
        unsafe { sys::LLVMRustTargetHasJIT(self.0) }
    }
}

impl SafeWrapper for Target
{
    type Inner = sys::TargetRef;

    unsafe fn from_inner(inner: sys::TargetRef) -> Self { Target(inner) }
    fn inner(&self) -> sys::TargetRef { self.0 }
}

impl fmt::Debug for Target
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Target({} - \"{}\")", self.name(), self.short_description())
    }
}

impl CPU
{
    /// Gets the CPU name of the host CPU.
    pub fn native() -> Self { CPU("native".to_owned()) }
}

impl Default for Triple
{
    fn default() -> Self {
        let s = unsafe { sys::DefaultTargetTriple() };
        Triple(s.as_string().expect("default target triple is not valid utf-8"))
    }
}

impl Default for CPU
{
    fn default() -> Self { CPU("".to_owned()) }
}

impl fmt::Display for Triple {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}
