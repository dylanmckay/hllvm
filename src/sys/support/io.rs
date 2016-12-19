use RawPWriteStreamRef;
use libc;

cpp! {
    #include "llvm/Support/raw_ostream.h"

    pub fn LLVMRustCreateRawFdOStream(fd: libc::c_int as "int",
                                      should_close: bool as "bool",
                                      unbuffered: bool as "bool")
        -> RawPWriteStreamRef as "llvm::raw_fd_ostream*" {
        return new llvm::raw_fd_ostream(fd, should_close, unbuffered);
    }

    pub fn LLVMRustDestroyRawPWriteStream(stream: RawPWriteStreamRef as "llvm::raw_fd_ostream*") {
        delete stream;
    }
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
