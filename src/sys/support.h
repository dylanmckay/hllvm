#pragma once

#include <llvm/Support/Casting.h>

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

} // end namespace support
