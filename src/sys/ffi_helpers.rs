use libc;

use std::ffi::CStr;
use std::str;

/// An owned C string.
/// NOT a `std::string`.
#[repr(C, packed)]
pub struct CppString(CppBox<libc::c_char>);

/// A boxed value, allocated from C++.
/// We own the value and free it ourselves.
#[repr(C, packed)]
pub struct CppBox<T>
{
    ptr: *mut T,
}

impl<T> Drop for CppBox<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut libc::c_void) }
    }
}

// We don't allow copying because that could lead to double-frees.
impl<T: Clone> Clone for CppBox<T> {
    fn clone(&self) -> Self {
        use std::mem::size_of;
        use std::ptr;

        unsafe {
            let new_ptr = libc::malloc(size_of::<T>()) as *mut T;
            ptr::copy(self.ptr, new_ptr, size_of::<T>());

            CppBox { ptr: new_ptr }
        }
    }
}

impl CppString {
    /// Gets the C-representation of the string.
    pub fn as_cstr<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.0.ptr) }
    }

    /// Convert the string into a Rust string.
    pub fn as_string(&self) -> Result<String, str::Utf8Error> {
        self.as_cstr().to_str().map(ToOwned::to_owned)
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn test_slice_repr_hasnt_changed() {
        let ptr = 0xDEADBEEF as *const u8;
        let len = 0xCAFEBABE;

        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, len);
            assert_eq!(slice.as_ptr(), ptr);
            assert_eq!(slice.len(), len);

            #[repr(C)]
            struct Slice { ptr: *const u8, len: usize }
            let slice: Slice = ::std::mem::transmute(slice);
            assert_eq!(slice.ptr, ptr);
            assert_eq!(slice.len, len);
        }
    }
}
