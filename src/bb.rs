//! A wrapper around a `LLVMBasicBlockRef`

use super::*;

/// A wrapper around a `LLVMBasicBlockRef`
#[derive(Copy, Clone)]
pub struct BasicBlock {
    pub(crate) basic_block: LLVMBasicBlockRef
}

impl BasicBlock {
    /// Delete this basic block
    pub fn delete(self) {
        unsafe {
            LLVMDeleteBasicBlock(self.basic_block)
        }
    }

    /// Get the name of a basic block
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_c(LLVMGetBasicBlockName(self.basic_block))
        }
    }

    /// Returns the internal basic block reference
    pub unsafe fn inner(&self) -> LLVMBasicBlockRef {
        self.basic_block
    }
}

impl Deref for BasicBlock {
    type Target = LLVMBasicBlockRef;

    fn deref(&self) -> &LLVMBasicBlockRef {
        &self.basic_block
    }
}

impl Debug for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.get_name() {
            write!(f, "BasicBlock({})", name)
        } else {
            write!(f, "BasicBlock")
        }
    }
}