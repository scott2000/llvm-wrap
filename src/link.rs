//! A renamed `LLVMLinkage`
use super::*;

/// A renamed `LLVMLinkage`
///
/// *Some deprecated values have been removed, see `LLVMLinkage`*
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Linkage {
    /// The default, externally visible linkage type
    External = 0,
    /// Globals will not be emitted to the object file and definitions will be used for
    /// optimization purposes, allowing inlining and discarding
    AvailableExternally = 1,
    /// Globals are merged with other globals of the same name during linkage and unused globals
    /// are discarded
    LinkOnceAny = 2,
    /// Similar to `LinkOnceAny`, but it allows further optimizations by ensuring that only globals
    /// with equivalent definitions are merged
    LinkOnceODR = 3,
    /// Similar to `LinkOnceAny`, but unused globals are not discarded
    WeakAny = 5,
    /// Similar to `LinkOnceODR`, but unused globals are not discarded
    WeakODR = 6,
    /// Global arrays are appended together when linkage occurs
    Appending = 7,
    /// Similar to `Private`, but values are represented as local symbols
    Internal = 8,
    /// Globals are only directly accessible by objects in the current module
    Private = 9,
    /// The symbol is `Weak` until linked, otherwise it becomes null instead of undefined
    ExternalWeak = 12,
    /// Used for tentative definitions at global scope, similar to `Weak`
    Common = 14,
}

impl Linkage {
    /// The `LLVMLinkage` this value represents
    pub fn inner(&self) -> LLVMLinkage {
        use llvm_sys::LLVMLinkage::*;
        use self::Linkage::*;
        match self {
            &External => LLVMExternalLinkage,
            &AvailableExternally => LLVMAvailableExternallyLinkage,
            &LinkOnceAny => LLVMLinkOnceAnyLinkage,
            &LinkOnceODR => LLVMLinkOnceODRLinkage,
            &WeakAny => LLVMWeakAnyLinkage,
            &WeakODR => LLVMWeakODRLinkage,
            &Appending => LLVMAppendingLinkage,
            &Internal => LLVMInternalLinkage,
            &Private => LLVMPrivateLinkage,
            &ExternalWeak => LLVMExternalWeakLinkage,
            &Common => LLVMCommonLinkage,
        }
    }
}