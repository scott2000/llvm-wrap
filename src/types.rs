//! Provides functions that return LLVM types in the global context

use super::*;
use super::c_api::*;

/// Create a named struct type
pub fn create_named_struct<S>(name: S) -> Type where S: AsRef<str> {
    Type {
        ty: unsafe {
            LLVMStructCreateNamed(context(), into_c(name).as_ptr())
        }
    }
}

/// A array type with a certain number of elements
pub fn ty_array(ty: Type, count: u32) -> Type {
    Type {
        ty: unsafe {
            LLVMArrayType(ty.ty, count)
        }
    }
}

/// A struct type with the given elements
pub fn ty_struct(elements: Vec<Type>, packed: bool) -> Type {
    Type {
        ty: unsafe {
            LLVMStructTypeInContext(context(), ty_vec(&elements).as_mut_ptr(), elements.len() as u32, packed as i32)
        }
    }
}

/// The `void` type
pub fn ty_void() -> Type {
    Type {
        ty: unsafe {
            LLVMVoidTypeInContext(context())
        }
    }
}

/// The `i1` type
pub fn ty_i1() -> Type {
    Type {
        ty: unsafe {
            LLVMInt1TypeInContext(context())
        }
    }
}

/// The `i8` type
pub fn ty_i8() -> Type {
    Type {
        ty: unsafe {
            LLVMInt8TypeInContext(context())
        }
    }
}

/// The `i16` type
pub fn ty_i16() -> Type {
    Type {
        ty: unsafe {
            LLVMInt16TypeInContext(context())
        }
    }
}

/// The `i32` type
pub fn ty_i32() -> Type {
    Type {
        ty: unsafe {
            LLVMInt32TypeInContext(context())
        }
    }
}

/// The `i64` type
pub fn ty_i64() -> Type {
    Type {
        ty: unsafe {
            LLVMInt64TypeInContext(context())
        }
    }
}

/// The `i128` type
pub fn ty_i128() -> Type {
    Type {
        ty: unsafe {
            LLVMInt128TypeInContext(context())
        }
    }
}

/// The `isize` type for a given data layout
pub fn ty_isize(data: &target::TargetData) -> Type {
    Type {
        ty: unsafe {
            llvm_sys::target::LLVMIntPtrTypeInContext(context(), data.data)
        }
    }
}

/// An integer type with any number of bits
pub fn ty_i(bits: u32) -> Type {
    Type {
        ty: unsafe {
            LLVMIntTypeInContext(context(), bits)
        }
    }
}

/// The `half` type
pub fn ty_half() -> Type {
    Type {
        ty: unsafe {
            LLVMHalfTypeInContext(context())
        }
    }
}

/// The `float` type
pub fn ty_float() -> Type {
    Type {
        ty: unsafe {
            LLVMFloatTypeInContext(context())
        }
    }
}

/// The `double` type
pub fn ty_double() -> Type {
    Type {
        ty: unsafe {
            LLVMDoubleTypeInContext(context())
        }
    }
}

/// The `fp128` type
pub fn ty_fp128() -> Type {
    Type {
        ty: unsafe {
            LLVMFP128TypeInContext(context())
        }
    }
}