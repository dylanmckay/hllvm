#pragma once

#include <llvm/ADT/ArrayRef.h>
#include <llvm/Support/Casting.h>

#include <stdint.h>
#include <string.h>

/// Whether or not we should perform expensive checks.
#define SUPPORT_DO_EXPENSIVE_CHECKS 1

namespace support {

/// Casts a value to another value.
///
/// If `DO_EXPENSIVE_CHECKS` is enabled, RTTI will be used to ensure
/// the cast is valid.
template<typename T, typename F>
T* cast(F* value) {
  if (!value) { return nullptr; }

#if SUPPORT_DO_EXPENSIVE_CHECKS
  T *to = llvm::dyn_cast<T>(value);
  assert(to && "object is not castable");;
#else
  T *to = static_cast<T>(value);
#endif
  return to;
}

/// A slice passed from Rust.
/// Must match Rust's internal representation of slices.
template<typename T>
class Slice
{
public:
  T *Ptr;
  uintptr_t Len;

  template<typename C>
  Slice<C> cast() {
#if SUPPORT_DO_EXPENSIVE_CHECKS
    for (T elem : this->ref()) {
      // FIXME: Implement this
      // assert(llvm::isa<C>(elem) && "not all elements are the correct class");
    }
#endif
    return Slice<C> { (C*) this->Ptr, this->Len};
  }

  llvm::ArrayRef<T> ref() {
    return llvm::ArrayRef<T>(this->Ptr, this->Len);
  }

  T &operator[](size_t index) { return this->Ptr[index]; }
  const T &operator[](size_t index) const { return this->Ptr[index]; }

  size_t len() const { return this->Len; }
};

/// Builds an owned `CppString` from a `std::string`.
inline char* BuildCppString(std::string &str) {
  char *owned = strdup(str.c_str());
  return owned;
}

} // end namespace support
