//! A safer wrapper for the LLVM C API bindings in Rust, based on `llvm-sys`
//!
//! ## Notes on Safety
//!
//! Although there are no `unsafe` blocks needed to use this library, it does ignore some of Rust's
//! constraints on memory management. Mainly, immutable references can still be modified in order
//! to make certain programs more simple.
//!
//! To make things simple, `LLVMContext`, `LLVMBuilder`, and `LLVMModule` are disposed
//! automatically when they leave scope. Additionally, values and types cannot outlive the context
//! in which they were created.
//!
//! If necessary, it is possible to use the `inner` function to access the wrapped value, or
//! `into_inner` to destroy the wrapper without disposing the contained value. Most types can also
//! dereference into their C equivalents. This crate is still in development and many features are
//! not fully supported, so use this if you need to use unsupported LLVM functions. Use `into_c`
//! and `from_c` to make strings easier to work with when dealing directly with the C API.
//!
//! ## Setup
//!
//! This crate relies heavily on `llvm-sys` and requires the same setup. Before using this crate,
//! be sure to install a version of LLVM 5 and add `LLVM_SYS_50_PREFIX=/path/to/llvm` to your `PATH`
//! environment variable.
//!
//! [LLVM Documentation] | [LLVM Language Reference] | [Rust Bindings]
//!
//! ## Example
//!
//! ```
//! # extern crate llvm_wrap as llvm;
//! # use llvm::*;
//! # use llvm::types::*;
//! # fn main() {
//! // Create a module
//! let module = create_module("add");
//! // Create a builder
//! let builder = create_builder();
//!
//! // Get an `i32 (i32, i32)` type
//! let ty = ty_i32().function(vec![ty_i32(); 2], false);
//! // Create the add function
//! let def = module.add_function("add", ty);
//! // Add an entry block
//! let entry = def.append_basic_block("entry");
//! // Move the builder to the end of the block
//! builder.position_at_end(entry);
//! // Add and name the two parameters
//! let result = builder.build_int_add(
//!     def.param(0).name("a"),
//!     def.param(1).name("b"),
//! );
//! // Return and name the result
//! builder.build_ret(result.name("tmp"));
//!
//! // Dump the contents of the module
//! module.dump();
//! # }
//! ```
//!
//! ## Output
//!
//! ```text
//! ; ModuleID = 'add'
//! source_filename = "add"
//!
//! define i32 @add(i32 %a, i32 %b) {
//! entry:
//!   %tmp = add i32 %a, %b
//!   ret i32 %tmp
//! }
//! ```
//!
//! [LLVM Documentation]: http://releases.llvm.org/5.0.0/docs/index.html
//! [LLVM Language Reference]: http://releases.llvm.org/5.0.0/docs/LangRef.html
//! [Rust Bindings]: http://rustdoc.taricorp.net/llvm-sys/llvm_sys

#![deny(missing_docs)]

extern crate llvm_sys;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::*;

use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::fmt::{self, Debug, Display};

mod context;
mod module;
mod builder;
mod ty;
mod bb;
mod val;
mod cc;
mod link;

pub mod iter;
pub mod target;
pub mod types;

/// Provides functions for using the C API
pub mod c_api {
    use super::*;

    /// Returns the global `LLVMContextRef`
    pub unsafe fn context() -> LLVMContextRef {
        LLVMGetGlobalContext()
    }

    /// Create a wrapper for a type
    pub unsafe fn ty(ty: LLVMTypeRef) -> Type {
        Type {
            ty,
        }
    }

    /// Create a wrapper for a value
    pub unsafe fn value(value: LLVMValueRef) -> Value {
        Value {
            value,
        }
    }

    /// Converts a `String` into a `CString`
    ///
    /// This function will also work with other, similar inputs like `&str` literals or
    /// `String` references. Use the `as_ptr` method to pass a pointer to a C function.
    pub fn into_c<S: AsRef<str>>(string: S) -> CString {
        CString::new(string.as_ref()).expect("invalid name")
    }

    /// Converts a `*const i8` into a `String`, if possible
    ///
    /// Returns `Some` if the pointer isn't null and points to a valid, non-empty string and
    /// returns `None` otherwise.
    pub fn from_c(string: *const i8) -> Option<String> {
        if !string.is_null() {
            unsafe {
                if let Ok(string) = CStr::from_ptr(string).to_str() {
                    if !string.is_empty() {
                        return Some(string.to_string());
                    }
                }
            }
        }
        None
    }

    #[doc(inline)]
    pub use llvm_sys::prelude::{LLVMValueRef, LLVMTypeRef, LLVMContextRef, LLVMBuilderRef, LLVMModuleRef, LLVMBasicBlockRef};
}

#[doc(inline)]
pub use context::*;
#[doc(inline)]
pub use module::Module;
#[doc(inline)]
pub use builder::Builder;
#[doc(inline)]
pub use ty::Type;
#[doc(inline)]
pub use bb::BasicBlock;
#[doc(inline)]
pub use val::Value;
#[doc(inline)]
pub use cc::CallConv;
#[doc(inline)]
pub use link::Linkage;

/// Converts a `Vec<Value>` into a `Vec<LLVMValueRef>`
fn val_vec(vals: &Vec<Value>) -> Vec<LLVMValueRef> {
    vals.iter().map(|i| i.value).collect()
}

/// Converts a `Vec<Type>` into a `Vec<LLVMTypeRef>`
fn ty_vec(types: &Vec<Type>) -> Vec<LLVMTypeRef> {
    types.iter().map(|i| i.ty).collect()
}