//! A wrapper around a `LLVMModuleRef` for a specific context

use super::*;
use super::c_api::*;

use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use std::path::Path;

/// A wrapper around a `LLVMModuleRef` for a specific context
pub struct Module {
    pub(crate) module: Option<LLVMModuleRef>,
}

impl Module {
    /// Add a function to the module
    pub fn add_function<S>(&self, name: S, ty: Type) -> Value where S: AsRef<str> {
        Value {
            value: unsafe {
                LLVMAddFunction(self.module.unwrap(), into_c(name).as_ptr(), ty.ty)
            }
        }
    }

    /// Add a global to the module
    pub fn add_global<S>(&self, name: S, ty: Type) -> Value where S: AsRef<str> {
        Value {
            value: unsafe {
                LLVMAddGlobal(self.module.unwrap(), ty.ty, into_c(name).as_ptr())
            }
        }
    }

    /// Get the function with the given name
    pub fn get_function<S>(&self, name: S) -> Value where S: AsRef<str> {
        Value {
            value: unsafe {
                LLVMGetNamedFunction(self.module.unwrap(), into_c(name).as_ptr())
            }
        }
    }

    /// Get the global with the given name
    pub fn get_global<S>(&self, name: S) -> Value where S: AsRef<str> {
        Value {
            value: unsafe {
                LLVMGetNamedGlobal(self.module.unwrap(), into_c(name).as_ptr())
            }
        }
    }

    /// Sets the target triple for this module
    pub fn set_triple<S>(&self, triple: S) where S: AsRef<str> {
        unsafe {
            LLVMSetTarget(self.module.unwrap(), into_c(triple).as_ptr())
        }
    }

    /// Sets the data layout for this module
    pub fn set_data_layout(&self, data: &target::TargetData) {
        unsafe {
            LLVMSetDataLayout(self.module.unwrap(), into_c(data.to_string()).as_ptr())
        }
    }

    /// Returns an iterator over all functions in the module
    pub fn functions(&self) -> iter::Functions {
        iter::Functions {
            pointer: Value {
                value: unsafe {
                    LLVMGetFirstFunction(self.module.unwrap())
                }
            }
        }
    }

    /// Returns an iterator over all globals in the module
    pub fn globals(&self) -> iter::Globals {
        iter::Globals {
            pointer: Value {
                value: unsafe {
                    LLVMGetFirstGlobal(self.module.unwrap())
                }
            }
        }
    }

    /// Dump the contents of the module to stderr
    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.module.unwrap());
        }
    }

    /// Write module IR to a file
    pub fn write_llvm_ir<P>(&self, path: P) where P: AsRef<Path> {
        unsafe {
            if LLVMPrintModuleToFile(
                self.module.unwrap(),
                into_c(path.as_ref()
                    .to_str()
                    .expect("path could not be converted to string")
                ).as_ptr(),
                vec![into_c("could not output LLVM IR for module").as_ptr() as *mut i8].as_mut_ptr(),
            ) != 0 {
                panic!("failed to write LLVM IR to file");
            }
        }
    }

    /// Write module bitcode to a file
    pub fn write_bitcode<P>(&self, path: P) where P: AsRef<Path> {
        unsafe {
            if LLVMWriteBitcodeToFile(
                self.module.unwrap(),
                into_c(path.as_ref()
                    .to_str()
                    .expect("path could not be converted to string")
                ).as_ptr()
            ) != 0 {
                panic!("failed to write bitcode to file");
            }
        }
    }

    /// Returns the internal module reference
    pub fn inner(&self) -> LLVMModuleRef {
        self.module.unwrap()
    }

    /// Destroys the wrapper, returning the internal module reference
    pub unsafe fn into_inner(mut self) -> LLVMModuleRef {
        self.module.take().unwrap()
    }
}

impl Deref for Module {
    type Target = LLVMModuleRef;

    fn deref(&self) -> &LLVMModuleRef {
        self.module.as_ref().unwrap()
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if let Some(module) = self.module {
            unsafe {
                LLVMDisposeModule(module);
            }
        }
    }
}

impl Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Module")
    }
}