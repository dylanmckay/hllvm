#pragma once

//! Core C++ include file.

#include <llvm/ADT/ArrayRef.h>

/// An array which can be passed from Rust to C++.
/// This is basically identical to a slice, but we can
/// guarantee the layout.
template<typename T>
class Array
{
public:
  T *Ptr;
  size_t Count;

  // Gets an LLVM array reference.
  llvm::ArrayRef<T> ref() {
    return llvm::ArrayRef<T>(this->Ptr, this->Count);
  }
} __attribute__((packed));
