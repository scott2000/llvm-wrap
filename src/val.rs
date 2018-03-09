//! A wrapper around a `LLVMValueRef`

use super::*;
use super::c_api::*;

/// A wrapper around a `LLVMValueRef` for a specific context
#[derive(Copy, Clone)]
pub struct Value {
    pub(crate) value: LLVMValueRef
}

impl Value {
    /// Adds a block to this function
    pub fn append_basic_block<S>(&self, name: S) -> BasicBlock where S: AsRef<str> {
        BasicBlock {
            basic_block: unsafe {
                LLVMAppendBasicBlockInContext(context(), self.value, CString::new(name.as_ref()).expect("invalid module name").as_ptr() as *const i8)
            }
        }
    }

    /// Delete this function
    pub fn delete_function(self) {
        unsafe {
            LLVMDeleteFunction(self.value)
        }
    }

    /// Delete this global
    pub fn delete_global(self) {
        unsafe {
            LLVMDeleteGlobal(self.value)
        }
    }

    /// Set the calling convention of this function
    pub fn set_call_conv(&self, cc: CallConv) -> Value {
        unsafe {
            LLVMSetFunctionCallConv(self.value, cc as u32);
        }
        *self
    }

    /// Set the linkage of this global
    pub fn set_linkage(&self, link: Linkage) -> Value {
        unsafe {
            LLVMSetLinkage(self.value, link.inner());
        }
        *self
    }

    /// Set whether this is a tail call
    pub fn set_tail_call(&self, tail: bool) -> Value {
        unsafe {
            LLVMSetTailCall(self.value, tail as i32);
        }
        *self
    }

    /// Set whether this global is a constant
    pub fn set_global_const(&self, constant: bool) -> Value {
        unsafe {
            LLVMSetGlobalConstant(self.value, constant as i32);
        }
        *self
    }

    /// Set whether the address of this global is significant
    pub fn set_unnamed_addr(&self, unnamed_addr: bool) -> Value {
        unsafe {
            LLVMSetUnnamedAddr(self.value, unnamed_addr as i32);
        }
        *self
    }

    /// Set the initializer of this global
    pub fn set_global_initializer(&self, init: Value) -> Value {
        unsafe {
            LLVMSetInitializer(self.value, init.value);
        }
        *self
    }

    /// Set the alignment of this value
    pub fn set_alignment(&self, bytes: u32) -> Value {
        unsafe {
            LLVMSetAlignment(self.value, bytes);
        }
        *self
    }

    /// Get a parameter for this function
    pub fn param(&self, param: u32) -> Value {
        Value {
            value: unsafe {
                LLVMGetParam(self.value, param as u32)
            }
        }
    }

    /// Returns an iterator over all parameters in this function
    pub fn params(&self) -> iter::Params {
        iter::Params {
            pointer: Value {
                value: unsafe {
                    LLVMGetFirstParam(self.value)
                }
            }
        }
    }

    /// Set the alignment of this parameter
    pub fn set_param_alignment(&self, bytes: u32) -> Value {
        unsafe {
            LLVMSetParamAlignment(self.value, bytes);
        }
        *self
    }

    /// Returns an iterator over all basic blocks in this function
    pub fn blocks(&self) -> iter::Blocks {
        iter::Blocks {
            pointer: BasicBlock {
                basic_block: unsafe {
                    LLVMGetFirstBasicBlock(self.value)
                }
            }
        }
    }

    /// Set the name of a value
    pub fn name<S>(&self, name: S) -> Value where S: AsRef<str> {
        unsafe {
            LLVMSetValueName(self.value, into_c(name).as_ptr());
        }
        *self
    }

    /// Get the name of a value
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_c(LLVMGetValueName(self.value))
        }
    }

    /// Get the type of this value
    pub fn ty(&self) -> Type {
        Type {
            ty: unsafe {
                LLVMTypeOf(self.value)
            }
        }
    }

    /// Dump the contents of the value to stderr
    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.value);
        }
    }

    /// Returns the internal value reference
    pub fn inner(&self) -> LLVMValueRef {
        self.value
    }
}

impl Deref for Value {
    type Target = LLVMValueRef;

    fn deref(&self) -> &LLVMValueRef {
        &self.value
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.get_name() {
            write!(f, "Value({})", name)
        } else {
            write!(f, "Value")
        }
    }
}