//! A wrapper around a `LLVMTypeRef`

use super::*;
use std::mem;

/// A wrapper around a `LLVMTypeRef` for a specific context
#[derive(Copy, Clone)]
pub struct Type<'c> {
    pub(crate) context: &'c Context,
    pub(crate) ty: LLVMTypeRef
}

impl<'c> Type<'c> {
    /// Creates a function that returns this type
    pub fn function(&self, params: Vec<Type>, va_args: bool) -> Type<'c> {
        Type {
            context: self.context,
            ty: unsafe {
                LLVMFunctionType(self.ty, ty_vec(&params).as_mut_ptr(), params.len() as u32, va_args as i32)
            }
        }
    }

    /// Creates a pointer to this type
    pub fn pointer(&self) -> Type<'c> {
        Type {
            context: self.context,
            ty: unsafe {
                LLVMPointerType(self.ty, 0)
            }
        }
    }

    /// The internal reference counter
    pub fn rc(&self) -> Type<'c> {
        self.context.ty_struct(vec![*self, self.context.ty_i32()], false)
    }

    /// Set the body of a struct
    pub fn struct_set_body(&self, elements: Vec<Type<'c>>, packed: bool) {
        unsafe {
            LLVMStructSetBody(self.ty, ty_vec(&elements).as_mut_ptr(), elements.len() as u32, packed as i32)
        }
    }

    /// An integer constant of this type
    pub fn const_int(&self, val: u64) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstInt(self.ty, val, 0)
            }
        }
    }

    /// An integer constant of this type
    pub fn const_signed_int(&self, val: i64) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstInt(self.ty, mem::transmute(val), 0)
            }
        }
    }

    /// A real constant of this type
    pub fn const_real(&self, val: f64) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstReal(self.ty, val)
            }
        }
    }

    /// A constant named struct with the given elements
    pub fn const_struct(&self, elements: Vec<Value<'c>>) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstNamedStruct(self.ty, val_vec(&elements).as_mut_ptr(), elements.len() as u32)
            }
        }
    }

    /// A constant array with the given elements
    pub fn const_array(&self, elements: Vec<Value<'c>>) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstArray(self.ty, val_vec(&elements).as_mut_ptr(), elements.len() as u32)
            }
        }
    } //TODO add Functions, Globals, and Params in public module `iter`

    /// The `undef` value for this type
    pub fn undef(&self) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMGetUndef(self.ty)
            }
        }
    }

    /// The `null` value for this type
    pub fn null(&self) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstNull(self.ty)
            }
        }
    }

    /// The `null` value for this pointer type
    pub fn null_ptr(&self) -> Value<'c> {
        Value {
            context: self.context,
            value: unsafe {
                LLVMConstPointerNull(self.ty)
            }
        }
    }

    /// Dump the contents of the type to stderr
    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(self.ty);
        }
    }

    /// Returns the internal type reference
    pub fn inner(&self) -> LLVMTypeRef {
        self.ty
    }
}

impl<'c> Deref for Type<'c> {
    type Target = LLVMTypeRef;

    fn deref(&self) -> &LLVMTypeRef {
        &self.ty
    }
}

impl<'c> Debug for Type<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type")
    }
}