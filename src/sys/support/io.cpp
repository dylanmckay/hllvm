#include "../support.h"

#include <llvm/Support/raw_ostream.h>

extern "C" {
  llvm::raw_fd_ostream *LLVMRustCreateRawFdOStream(int fd,
                                                   bool shouldClose,
                                                   bool unbuffered) {
    return new llvm::raw_fd_ostream(fd, shouldClose, unbuffered);
  }

  void LLVMRustDestroyRawPWriteStream(llvm::raw_fd_ostream *OS) {
    delete OS;
  }
}
