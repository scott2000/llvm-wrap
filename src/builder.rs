//! A wrapper around a `LLVMBuilderRef` for a specific context

use super::*;
use super::types::*;
use super::c_api::*;

/// A wrapper around a `LLVMBuilderRef` for a specific context
pub struct Builder {
    pub(crate) builder: Option<LLVMBuilderRef>,
}

impl Builder {
    /// Build a stack allocation
    pub fn build_alloca(&self, ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildAlloca(self.builder.unwrap(), ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a heap allocation
    pub fn build_malloc(&self, ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildMalloc(self.builder.unwrap(), ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a stack allocation for an array
    pub fn build_array_alloca(&self, ty: Type, count: u32) -> Value {
        Value {
            value: unsafe {
                LLVMBuildArrayAlloca(self.builder.unwrap(), ty.ty, ty_i32().const_int(count as u64).value, into_c("").as_ptr())
            }
        }
    }

    /// Build a heap allocation for an array
    pub fn build_array_malloc(&self, ty: Type, count: u32) -> Value {
        Value {
            value: unsafe {
                LLVMBuildArrayMalloc(self.builder.unwrap(), ty.ty, ty_i32().const_int(count as u64).value, into_c("").as_ptr())
            }
        }
    }

    /// Build a heap free
    pub fn build_free(&self, ptr: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFree(self.builder.unwrap(), ptr.value)
            }
        }
    }

    /// Build a call to a function
    pub fn build_call(&self, func: Value, args: Vec<Value>) -> Value {
        Value {
            value: unsafe {
                LLVMBuildCall(self.builder.unwrap(), func.value, val_vec(&args).as_mut_ptr(), args.len() as u32, into_c("").as_ptr())
            }
        }
    }

    /// Build a struct initialization for the given type and elements
    pub fn build_struct_init(&self, ty: Type, elements: Vec<Value>) -> Value {
        let mut agg: Value = ty.undef();
        for (index, element) in elements.into_iter().enumerate() {
            agg = self.build_insert_value(agg, element, index as u32).name(format!("insert_{}", index));
        }
        agg
    }

    /// Build an insert value instruction
    pub fn build_insert_value(&self, agg: Value, elt: Value, index: u32) -> Value {
        Value {
            value: unsafe {
                LLVMBuildInsertValue(self.builder.unwrap(), agg.value, elt.value, index, into_c("").as_ptr())
            }
        }
    }

    /// Build an extract value instruction
    pub fn build_extract_value(&self, agg: Value, index: u32) -> Value {
        Value {
            value: unsafe {
                LLVMBuildExtractValue(self.builder.unwrap(), agg.value, index, into_c("").as_ptr())
            }
        }
    }

    /// Build a get element pointer instruction
    pub fn build_gep(&self, ptr: Value, indices: Vec<Value>) -> Value {
        Value {
            value: unsafe {
                LLVMBuildGEP(self.builder.unwrap(), ptr.value,
                             val_vec(&indices).as_mut_ptr(), indices.len() as u32, into_c("").as_ptr())
            }
        }
    }

    /// Build a struct get element pointer instruction
    pub fn build_struct_gep(&self, ptr: Value, index: u32) -> Value {
        Value {
            value: unsafe {
                LLVMBuildStructGEP(self.builder.unwrap(), ptr.value, index, into_c("").as_ptr())
            }
        }
    }

    /// Build an inbounds get element pointer instruction
    pub fn build_inbounds_gep(&self, ptr: Value, indices: Vec<Value>) -> Value {
        Value {
            value: unsafe {
                LLVMBuildGEP(self.builder.unwrap(), ptr.value,
                             val_vec(&indices).as_mut_ptr(), indices.len() as u32, into_c("").as_ptr())
            }
        }
    }

    /// Build a global string with the given value
    pub fn build_global_string<S>(&self, string: S) -> Value where S: AsRef<str> {
        Value {
            value: unsafe {
                LLVMBuildGlobalString(self.builder.unwrap(), into_c(string).as_ptr(), into_c("").as_ptr())
            }
        }
    }

    /// Build a global string pointer with the given value
    pub fn build_global_string_ptr<S>(&self, string: S) -> Value where S: AsRef<str> {
        Value {
            value: unsafe {
                LLVMBuildGlobalStringPtr(self.builder.unwrap(), into_c(string).as_ptr(), into_c("").as_ptr())
            }
        }
    }

    /// Build a store instruction
    pub fn build_store(&self, val: Value, ptr: Value) {
        unsafe {
            LLVMBuildStore(self.builder.unwrap(), val.value, ptr.value);
        }
    }

    /// Build a load instruction
    pub fn build_load(&self, ptr: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildLoad(self.builder.unwrap(), ptr.value, into_c("").as_ptr())
            }
        }
    }

    /// Alloca some memory for an array and then store values in it
    pub fn build_array_alloca_store(&self, ty: Type, elements: Vec<Value>) -> Value {
        let mut agg = ty_array(ty, elements.len() as u32).undef();
        for (i, element) in elements.into_iter().enumerate() {
            agg = self.build_insert_value(agg, element, i as u32).name(format!("insert_elem_{}", i));
        }
        self.build_pointer_cast(self.build_alloca_store(agg), ty.pointer())
    }

    /// Malloc some memory for an array and then store values in it
    pub fn build_array_malloc_store(&self, ty: Type, elements: Vec<Value>) -> Value {
        let mut agg = ty_array(ty, elements.len() as u32).undef();
        for (i, element) in elements.into_iter().enumerate() {
            agg = self.build_insert_value(agg, element, i as u32).name(format!("insert_elem_{}", i));
        }
        self.build_pointer_cast(self.build_malloc_store(agg), ty.pointer())
    }

    /// Alloca some memory and then store a value in it
    pub fn build_alloca_store(&self, val: Value) -> Value {
        let ptr = self.build_alloca(val.ty());
        self.build_store(val, ptr);
        ptr
    }

    /// Malloc some memory and then store a value in it
    pub fn build_malloc_store(&self, val: Value) -> Value {
        let ptr = self.build_malloc(val.ty());
        self.build_store(val, ptr);
        ptr
    }

    /// Load a value and then free the memory
    pub fn build_load_free(&self, ptr: Value) -> Value {
        let val = self.build_load(ptr);
        self.build_free(ptr);
        val
    }

    /// Build a cast from integer to pointer
    pub fn build_int_to_ptr(&self, val: Value, ptr_ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildIntToPtr(self.builder.unwrap(), val.value, ptr_ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a cast from pointer to integer
    pub fn build_ptr_to_int(&self, ptr: Value, val_ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildPtrToInt(self.builder.unwrap(), ptr.value, val_ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a pointer cast
    pub fn build_pointer_cast(&self, ptr: Value, ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildPointerCast(self.builder.unwrap(), ptr.value, ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build an integer cast
    pub fn build_int_cast(&self, val: Value, ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildIntCast(self.builder.unwrap(), val.value, ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a bit cast
    pub fn build_bit_cast(&self, val: Value, ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildBitCast(self.builder.unwrap(), val.value, ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a floating point cast
    pub fn build_float_cast(&self, val: Value, ty: Type) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFPCast(self.builder.unwrap(), val.value, ty.ty, into_c("").as_ptr())
            }
        }
    }

    /// Build a `ret void` statement
    pub fn build_ret_void(&self) -> Value {
        Value {
            value: unsafe {
                LLVMBuildRetVoid(self.builder.unwrap())
            }
        }
    }

    /// Build a `ret` statement
    pub fn build_ret(&self, val: Value) -> Value where {
        Value {
            value: unsafe {
                LLVMBuildRet(self.builder.unwrap(), val.value)
            }
        }
    }

    /// Build an unreachable instruction
    pub fn build_unreachable(&self) {
        unsafe {
            LLVMBuildUnreachable(self.builder.unwrap());
        }
    }

    /// Build a branch instruction to the given block
    pub fn build_br(&self, block: BasicBlock) {
        unsafe {
            LLVMBuildBr(self.builder.unwrap(), block.basic_block);
        }
    }

    /// Build an if statement that branches to the given blocks
    pub fn build_if(&self, condition: Value, then_block: BasicBlock, else_block: BasicBlock) {
        unsafe {
            LLVMBuildCondBr(self.builder.unwrap(), condition.value, then_block.basic_block, else_block.basic_block);
        }
    }

    /// Build a switch statement that branches to the given blocks
    pub fn build_switch(&self, val: Value, cases: Vec<(Value, BasicBlock)>, default: BasicBlock) {
        unsafe {
            let switch = LLVMBuildSwitch(self.builder.unwrap(), val.value, default.basic_block, cases.len() as u32);
            for (val, bb) in cases {
                LLVMAddCase(switch, val.value, bb.basic_block);
            }
        }
    }

    /// Build a phi instruction that takes ceratin values from certain blocks
    pub fn build_phi(&self, incoming: Vec<(Value, BasicBlock)>) -> Value {
        Value {
            value: unsafe {
                if incoming.is_empty() {
                    panic!("phi node must have an incoming block list");
                } else {
                    let phi = LLVMBuildPhi(
                        self.builder.unwrap(),
                        incoming[0].0.ty().ty,
                        into_c("").as_ptr()
                    );
                    let len = incoming.len();
                    let mut values = Vec::new();
                    let mut blocks = Vec::new();
                    for (val, block) in incoming {
                        values.push(val.value);
                        blocks.push(block.basic_block);
                    }
                    LLVMAddIncoming(phi, values.as_mut_ptr(), blocks.as_mut_ptr(), len as u32);
                    phi
                }
            }
        }
    }

    /// Position the builder at a given value in a basic block
    pub fn position_in_block(&self, bb: BasicBlock, val: Value) {
        unsafe {
            LLVMPositionBuilder(self.builder.unwrap(), bb.basic_block, val.value);
        }
    }

    /// Position the builder at the end of the basic block
    pub fn position_at_end(&self, bb: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder.unwrap(), bb.basic_block);
        }
    }

    /// Position the builder before a value
    pub fn position_before(&self, val: Value) {
        unsafe {
            LLVMPositionBuilderBefore(self.builder.unwrap(), val.value);
        }
    }

    /// Returns the internal builder reference
    pub fn inner(&self) -> LLVMBuilderRef {
        self.builder.unwrap()
    }

    /// Destroys the wrapper, returning the internal builder reference
    pub unsafe fn into_inner(mut self) -> LLVMBuilderRef {
        self.builder.take().unwrap()
    }

    /// Builds a null check
    pub fn build_is_null(&self, val: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildIsNull(self.builder.unwrap(), val.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a null check
    pub fn build_is_not_null(&self, val: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildIsNotNull(self.builder.unwrap(), val.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `add` instruction
    pub fn build_int_add(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildAdd(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `sub` instruction
    pub fn build_int_sub(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildSub(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `mul` instruction
    pub fn build_int_mul(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildMul(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `udiv` instruction
    pub fn build_int_udiv(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildUDiv(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `sdiv` instruction
    pub fn build_int_sdiv(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildSDiv(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `urem` instruction
    pub fn build_int_urem(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildSRem(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `srem` instruction
    pub fn build_int_srem(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildSRem(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `eq` check
    pub fn build_int_eq(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntEQ, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `ne` check
    pub fn build_int_ne(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntNE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `ule` check
    pub fn build_int_ule(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntULE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `ult` check
    pub fn build_int_ult(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntULT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `uge` check
    pub fn build_int_uge(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntUGE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `ugt` check
    pub fn build_int_ugt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntUGT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `sle` check
    pub fn build_int_sle(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntSLE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `slt` check
    pub fn build_int_slt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntSLT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `sge` check
    pub fn build_int_sge(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntSGE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an integer `sgt` check
    pub fn build_int_sgt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildICmp(self.builder.unwrap(), LLVMIntPredicate::LLVMIntSGT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `add` instruction
    pub fn build_float_add(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFAdd(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `sub` instruction
    pub fn build_float_sub(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFSub(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `mul` instruction
    pub fn build_float_mul(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFMul(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `div` instruction
    pub fn build_float_div(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFDiv(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `rem` instruction
    pub fn build_float_rem(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFRem(self.builder.unwrap(), a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `eq` check
    pub fn build_float_eq(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealUEQ, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `ne` check
    pub fn build_float_ne(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealUNE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `le` check
    pub fn build_float_le(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealULE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `lt` check
    pub fn build_float_lt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealULT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `ge` check
    pub fn build_float_ge(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealUGE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a float `gt` check
    pub fn build_float_gt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealUGT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an ordered float `eq` check
    pub fn build_float_ord_eq(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealOEQ, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an ordered float `ne` check
    pub fn build_float_ord_ne(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealONE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an ordered float `le` check
    pub fn build_float_ord_le(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealOLE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an ordered float `lt` check
    pub fn build_float_ord_lt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealOLT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an ordered float `ge` check
    pub fn build_float_ord_ge(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealOGE, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds an ordered float `gt` check
    pub fn build_float_ord_gt(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealOGT, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a check for an ordered float
    pub fn build_float_is_ord(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealORD, a.value, b.value, into_c("").as_ptr())
            }
        }
    }

    /// Builds a check for an unordered float
    pub fn build_float_non_ord(&self, a: Value, b: Value) -> Value {
        Value {
            value: unsafe {
                LLVMBuildFCmp(self.builder.unwrap(), LLVMRealPredicate::LLVMRealUNO, a.value, b.value, into_c("").as_ptr())
            }
        }
    }
}

impl Deref for Builder {
    type Target = LLVMBuilderRef;

    fn deref(&self) -> &LLVMBuilderRef {
        self.builder.as_ref().unwrap()
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        if let Some(builder) = self.builder {
            unsafe {
                LLVMDisposeBuilder(builder);
            }
        }
    }
}

impl Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Builder")
    }
}