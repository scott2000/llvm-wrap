//! A wrapper around a `LLVMContextRef`

use super::*;

/// A wrapper around a `LLVMContextRef`
pub struct Context {
    pub(crate) context: Option<LLVMContextRef>
}

impl Context {
    /// Creates a new context
    pub fn new() -> Context {
        Context {
            context: Some(
                unsafe {
                    LLVMContextCreate()
                }
            ),
        }
    }

    /// Creates a new module with the given name in this context
    pub fn create_module<'c, S>(&'c self, name: S) -> Module<'c> where S: AsRef<str> {
        Module {
            context: self,
            module: Some(
                unsafe {
                    LLVMModuleCreateWithNameInContext(into_c(name).as_ptr(), self.context.unwrap())
                }
            )
        }
    }

    /// Creates a new builder in this context
    pub fn create_builder<'c>(&'c self) -> Builder<'c> {
        Builder {
            context: self,
            builder: Some(
                unsafe {
                    LLVMCreateBuilderInContext(self.context.unwrap())
                }
            )
        }
    }

    /// Create a named struct type
    pub fn create_named_struct<'c, S>(&'c self, name: S) -> Type<'c> where S: AsRef<str> {
        Type {
            context: self,
            ty: unsafe {
                LLVMStructCreateNamed(self.context.unwrap(), into_c(name).as_ptr())
            }
        }
    }

    /// A array type with a certain number of elements
    pub fn ty_array<'c>(&'c self, ty: Type<'c>, count: u32) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMArrayType(ty.ty, count)
            }
        }
    }

    /// A struct type with the given elements
    pub fn ty_struct<'c>(&'c self, elements: Vec<Type<'c>>, packed: bool) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMStructTypeInContext(self.context.unwrap(), ty_vec(&elements).as_mut_ptr(), elements.len() as u32, packed as i32)
            }
        }
    }

    /// A constant struct with the given elements
    pub fn const_struct<'c>(&'c self, elements: Vec<Value<'c>>, packed: bool) -> Value<'c> {
        Value {
            context: self,
            value: unsafe {
                LLVMConstStructInContext(self.context.unwrap(), val_vec(&elements).as_mut_ptr(), elements.len() as u32, packed as i32)
            }
        }
    }

    /// The `void` type
    pub fn ty_void<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMVoidTypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `i1` type
    pub fn ty_i1<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMInt1TypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `i8` type
    pub fn ty_i8<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMInt8TypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `i16` type
    pub fn ty_i16<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMInt16TypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `i32` type
    pub fn ty_i32<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMInt32TypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `i64` type
    pub fn ty_i64<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMInt64TypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `i128` type
    pub fn ty_i128<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMInt128TypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `isize` type for a given data layout
    pub fn ty_isize<'c>(&'c self, data: &target::TargetData) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                llvm_sys::target::LLVMIntPtrTypeInContext(self.context.unwrap(), data.data)
            }
        }
    }

    /// An integer type with any number of bits
    pub fn ty_i<'c>(&'c self, bits: u32) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMIntTypeInContext(self.context.unwrap(), bits)
            }
        }
    }

    /// The `half` type
    pub fn ty_half<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMHalfTypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `float` type
    pub fn ty_float<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMFloatTypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `double` type
    pub fn ty_double<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMDoubleTypeInContext(self.context.unwrap())
            }
        }
    }

    /// The `fp128` type
    pub fn ty_fp128<'c>(&'c self) -> Type<'c> {
        Type {
            context: self,
            ty: unsafe {
                LLVMFP128TypeInContext(self.context.unwrap())
            }
        }
    }

    /// Returns the internal context reference
    pub fn inner(&self) -> LLVMContextRef {
        self.context.unwrap()
    }

    /// Create a wrapper for a type
    pub unsafe fn ty<'c>(&'c self, ty: LLVMTypeRef) -> Type<'c> {
        Type {
            context: self,
            ty,
        }
    }

    /// Create a wrapper for a value
    pub unsafe fn value<'c>(&'c self, value: LLVMValueRef) -> Value<'c> {
        Value {
            context: self,
            value,
        }
    }

    /// Destroys the wrapper, returning the internal context reference
    pub unsafe fn into_inner(mut self) -> LLVMContextRef {
        self.context.take().unwrap()
    }
}

impl Deref for Context {
    type Target = LLVMContextRef;

    fn deref(&self) -> &LLVMContextRef {
        self.context.as_ref().unwrap()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if let Some(context) = self.context {
            unsafe {
                LLVMContextDispose(context);
            }
        }
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context")
    }
}