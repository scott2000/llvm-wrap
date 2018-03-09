//! A wrapper around the LLVM global context

use super::*;
use super::c_api::*;

/// Creates a new module with the given name in this context
pub fn create_module<S>(name: S) -> Module where S: AsRef<str> {
    Module {
        module: Some(
            unsafe {
                LLVMModuleCreateWithNameInContext(into_c(name).as_ptr(), context())
            }
        )
    }
}

/// Creates a new builder in this context
pub fn create_builder() -> Builder {
    Builder {
        builder: Some(
            unsafe {
                LLVMCreateBuilderInContext(context())
            }
        )
    }
}

/// A constant struct with the given elements
pub fn const_struct(elements: Vec<Value>, packed: bool) -> Value {
    Value {
        value: unsafe {
            LLVMConstStructInContext(context(), val_vec(&elements).as_mut_ptr(), elements.len() as u32, packed as i32)
        }
    }
}

/// A constant string with the given value
pub fn const_string<S>(string: S, null_terminated: bool) -> Value where S: AsRef<str> {
    Value {
        value: unsafe {
            let string = string.as_ref();
            LLVMConstStringInContext(context(), into_c(string).as_ptr(), string.len() as u32, null_terminated as i32)
        }
    }
}