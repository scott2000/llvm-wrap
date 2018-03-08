//! Wrappers for attributes and attribute indices
//!
//! *Not currently used because the LLVM API has a complicated interface for attributes*
use super::*;

/// An index representing the location of an attribute
#[derive(Copy, Clone, Debug)]
pub enum AttributeIndex {
    /// An attribute for a function
    Function,
    /// An attribute for a parameter of a function
    Parameter(u32),
    /// An attribute for a return of a function
    Return,
}

impl AttributeIndex {
    pub unsafe fn inner(&self) -> u32 {
        use self::AttributeIndex::*;
        match self {
            &Function => LLVMAttributeFunctionIndex,
            &Parameter(i) => i+1,
            &Return => LLVMAttributeReturnIndex,
        }
    }
}