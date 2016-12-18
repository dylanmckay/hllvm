use RawPWriteStreamRef;
use libc;

extern "C" {
    pub fn LLVMRustCreateRawFdOStream(fd: libc::c_int,
                                      should_close: bool,
                                      unbuffered: bool) -> RawPWriteStreamRef;

    pub fn LLVMRustDestroyRawPWriteStream(_: RawPWriteStreamRef);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_and_destroy_fd_stream() {
        unsafe {
            let stream = LLVMRustCreateRawFdOStream(0, false, true);
            LLVMRustDestroyRawPWriteStream(stream);
        }
    }
}
