//! Provides iterators for various items
use super::*;

/// An iterator over functions in a module
#[derive(Clone, Debug)]
pub struct Functions {
    pub(crate) pointer: Value,
}

impl Iterator for Functions {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        if self.pointer.value.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = Value {
                value: unsafe {
                    LLVMGetNextFunction(self.pointer.value)
                }
            };
            Some(next)
        }
    }
}

/// An iterator over global variables in a module
#[derive(Clone, Debug)]
pub struct Globals {
    pub(crate) pointer: Value,
}

impl Iterator for Globals {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        if self.pointer.value.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = Value {
                value: unsafe {
                    LLVMGetNextGlobal(self.pointer.value)
                }
            };
            Some(next)
        }
    }
}

/// An iterator over parameters in a function
#[derive(Clone, Debug)]
pub struct Params {
    pub(crate) pointer: Value,
}

impl Iterator for Params {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        if self.pointer.value.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = Value {
                value: unsafe {
                    LLVMGetNextParam(self.pointer.value)
                }
            };
            Some(next)
        }
    }
}

/// An iterator over basic blocks in a function
#[derive(Clone, Debug)]
pub struct Blocks {
    pub(crate) pointer: BasicBlock,
}

impl Iterator for Blocks {
    type Item = BasicBlock;

    fn next(&mut self) -> Option<BasicBlock> {
        if self.pointer.basic_block.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = BasicBlock {
                basic_block: unsafe {
                    LLVMGetNextBasicBlock(self.pointer.basic_block)
                }
            };
            Some(next)
        }
    }
}