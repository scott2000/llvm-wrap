//! A renamed `LLVMCallConv`
use super::*;

/// A renamed `LLVMCallConv`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CallConv {
    /// The default C calling convention
    C = 0,
    /// Makes calls as fast as possible. Allows tail calls
    Fast = 8,
    /// Assumes the function will not be called frequently
    Cold = 9,
    /// The calling convention for WebKit JS
    WebKitJS = 12,
    /// Dynamic calling convention for code patching
    AnyReg = 13,
    /// The x86 standard calling convention
    X86Stdcall = 64,
    /// The x86 fast calling convention
    X86Fastcall = 65,
}

impl CallConv {
    /// The `LLVMCallConv` this value represents
    pub fn inner(&self) -> LLVMCallConv {
        use llvm_sys::LLVMCallConv::*;
        use self::CallConv::*;
        match self {
            &C => LLVMCCallConv,
            &Fast => LLVMFastCallConv,
            &Cold => LLVMColdCallConv,
            &WebKitJS => LLVMWebKitJSCallConv,
            &AnyReg => LLVMAnyRegCallConv,
            &X86Stdcall => LLVMX86StdcallCallConv,
            &X86Fastcall => LLVMX86FastcallCallConv,
        }
    }
}