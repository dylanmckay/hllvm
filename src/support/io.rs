use SafeWrapper;
use sys;
use std::os::raw;
use std::os::unix::io::IntoRawFd;
use libc;

/// An abstract output stream.
pub struct OutputStream(sys::RawPWriteStreamRef);

/// An output stream backed by a file descriptor.
pub struct FileOutputStream(OutputStream);

impl FileOutputStream {
    /// Gets an output stream for standard output.
    pub fn stdout(unbuffered: bool) -> Self {
        unsafe { FileOutputStream::with_fd(libc::STDOUT_FILENO, false, unbuffered) }
    }

    /// Gets an output stream for standard error.
    pub fn stderr(unbuffered: bool) -> Self {
        unsafe { FileOutputStream::with_fd(libc::STDERR_FILENO, false, unbuffered) }
    }

    /// Creates an output stream from a given FD.
    ///
    /// This takes ownership of the FD, which will be closed when the stream
    /// is destroyed.
    pub fn from<T: IntoRawFd>(s: T, unbuffered: bool) -> Self {
        unsafe {
            FileOutputStream::with_fd(s.into_raw_fd(),
                                      true, unbuffered)
        }
    }

    /// Creates a file stream from a raw file descriptor.
    ///
    /// This should be used carefully.
    pub unsafe fn with_fd(fd: raw::c_int,
                          should_close: bool,
                          unbuffered: bool) -> Self {
        let inner = sys::LLVMRustCreateRawFdOStream(fd, should_close, unbuffered);
        FileOutputStream(OutputStream::from_inner(inner))
    }
}

impl SafeWrapper for OutputStream
{
    type Inner = sys::RawPWriteStreamRef;

    unsafe fn from_inner(inner: sys::RawPWriteStreamRef) -> Self { OutputStream(inner) }
    fn inner(&self) -> sys::RawPWriteStreamRef { self.0 }
}

impl Drop for OutputStream
{
    fn drop(&mut self) {
        unsafe {
            sys::LLVMRustDestroyRawPWriteStream(self.0);
        }
    }
}

impl AsRef<OutputStream> for FileOutputStream
{
    fn as_ref(&self) -> &OutputStream { &self.0 }
}
