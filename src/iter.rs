//! Provides iterators for various items
use super::*;

/// An iterator over functions in a module
#[derive(Clone, Debug)]
pub struct Functions<'c> {
    pub(crate) pointer: Value<'c>,
}

impl<'c> Iterator for Functions<'c> {
    type Item = Value<'c>;

    fn next(&mut self) -> Option<Value<'c>> {
        if self.pointer.value.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = Value {
                context: self.pointer.context,
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
pub struct Globals<'c> {
    pub(crate) pointer: Value<'c>,
}

impl<'c> Iterator for Globals<'c> {
    type Item = Value<'c>;

    fn next(&mut self) -> Option<Value<'c>> {
        if self.pointer.value.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = Value {
                context: self.pointer.context,
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
pub struct Params<'c> {
    pub(crate) pointer: Value<'c>,
}

impl<'c> Iterator for Params<'c> {
    type Item = Value<'c>;

    fn next(&mut self) -> Option<Value<'c>> {
        if self.pointer.value.is_null() {
            None
        } else {
            let next = self.pointer;
            self.pointer = Value {
                context: self.pointer.context,
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