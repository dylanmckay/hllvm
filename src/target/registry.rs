use SafeWrapper;
use target::Target;
use sys;

use std::{mem, marker};
use std::sync::Mutex;

lazy_static! {
    /// Keeps track of whether or not the registry has been initialized yet.
    static ref IS_INITIALIZED: Mutex<bool> = Mutex::new(false);
}

/// The target registry.
pub struct Registry {
    phantom: marker::PhantomData<()>,
}

impl Registry
{
    /// Gets the global target registry.
    pub fn get() -> Self {
        if !*IS_INITIALIZED.lock().unwrap() {
            unsafe { sys::LLVMRustInitializeAllTargets() }
        }

        Registry { phantom: marker::PhantomData }
    }

    /// Gets a list of all targets.
    pub fn targets(&self) -> ::std::vec::IntoIter<Target> {
        unsafe {
            let mut buffer: [sys::TargetRef; 100] = mem::uninitialized();

            let target_count = sys::LLVMRustTargetRegistryTargets(
                buffer.as_mut_ptr(), buffer.len());

            let target_refs = &buffer[0..target_count];

            let vec: Vec<_> = target_refs.iter().map(|&r| Target::from_inner(r)).collect();
            vec.into_iter()
        }
    }
}
