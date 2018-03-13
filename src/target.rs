//! Provides wrappers for target-related operations
use super::*;
use super::c_api::*;

use std::path::Path;
use std::ptr::null_mut;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;

static mut UNINITIALIZED: bool = true;

unsafe fn initialize() {
    if UNINITIALIZED {
        UNINITIALIZED = false;
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargetMCs();
    }
}

/// The default target triple
pub fn default_triple() -> String {
    from_c(unsafe {
        LLVMGetDefaultTargetTriple()
    }).unwrap_or(String::new())
}

/// A renamed `LLVMCodeGenOptLevel`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OptLevel {
    /// No optimization (`-O0`)
    None = 0,
    /// Less optimization (`-O1`)
    Less = 1,
    /// Default optimization (`-O2`)
    Default = 2,
    /// Aggressive optimization (`-O3`)
    Aggressive = 3,
}

impl OptLevel {
    /// The `LLVMCodeGenOptLevel` this value represents
    pub unsafe fn inner(&self) -> LLVMCodeGenOptLevel {
        use llvm_sys::target_machine::LLVMCodeGenOptLevel::*;
        use self::OptLevel::*;
        match self {
            &None => LLVMCodeGenLevelNone,
            &Less => LLVMCodeGenLevelLess,
            &Default => LLVMCodeGenLevelDefault,
            &Aggressive => LLVMCodeGenLevelAggressive,
        }
    }
}

/// A renamed `LLVMCodeGenFileType`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileType {
    /// The assembly (`.s`) file type
    Assembly,
    /// The object (`.o`) file type
    Object,
}

impl FileType {
    /// The `LLVMCodeGenFileType` this value represents
    pub fn inner(&self) -> LLVMCodeGenFileType {
        use llvm_sys::target_machine::LLVMCodeGenFileType::*;
        use self::FileType::*;
        match self {
            &Assembly => LLVMAssemblyFile,
            &Object => LLVMObjectFile,
        }
    }
}

/// A renamed `LLVMByteOrdering`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ByteOrdering {
    /// The most significant byte is stored first
    BigEndian,
    /// The least significant byte is stored first
    LittleEndian,
}

impl ByteOrdering {
    /// The `LLVMByteOrdering` this value represents
    pub fn inner(&self) -> LLVMByteOrdering {
        use llvm_sys::target::LLVMByteOrdering::*;
        use self::ByteOrdering::*;
        match self {
            &BigEndian => LLVMBigEndian,
            &LittleEndian => LLVMLittleEndian,
        }
    }
}

/// A wrapper around a `LLVMTargetRef`
#[derive(Copy, Clone)]
pub struct Target {
    target: LLVMTargetRef,
}

impl Target {
    /// Attempts to create a `Target` using the given triple
    pub fn from_triple(triple: String) -> Result<Target, String> {
        unsafe {
            initialize();
            let mut target: LLVMTargetRef = null_mut();
            let mut error = null_mut();
            if LLVMGetTargetFromTriple(
                    into_c(triple).as_ptr(),
                    &mut target as *mut LLVMTargetRef,
                    &mut error as *mut *mut i8,
                ) == 1 || target.is_null() {
                Err(from_c(error).unwrap_or(String::new()))
            } else {
                Ok(Target {
                    target,
                })
            }
        }
    }

    /// Creates a target machine with the default options
    pub fn create_machine(&self, triple: String) -> TargetMachine {
        self.create_machine_with_options(triple, "generic".to_owned(), String::new(), OptLevel::Default)
    }

    /// Creates a target machine with the given options
    pub fn create_machine_with_options(&self, triple: String, cpu: String, features: String,
                                       level: OptLevel) -> TargetMachine {
        TargetMachine {
            machine: unsafe {
                LLVMCreateTargetMachine(
                    self.target,
                    into_c(triple).as_ptr(),
                    into_c(cpu).as_ptr(),
                    into_c(features).as_ptr(),
                    level.inner(),
                    LLVMRelocMode::LLVMRelocDefault,
                    LLVMCodeModel::LLVMCodeModelDefault,
                )
            }
        }
    }

    /// Gets the name of this target
    pub fn name(&self) -> String {
        unsafe {
            from_c(LLVMGetTargetName(self.target)).unwrap_or(String::new())
        }
    }

    /// Gets the description of this target
    pub fn description(&self) -> String {
        unsafe {
            from_c(LLVMGetTargetDescription(self.target)).unwrap_or(String::new())
        }
    }
}

impl Debug for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Target({:?}, {:?})", self.name(), self.description())
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A wrapper around a `LLVMTargetMachineRef`
#[derive(Clone)]
pub struct TargetMachine {
    machine: LLVMTargetMachineRef,
}

impl TargetMachine {
    /// Creates a target machine with the native target triple and options
    pub fn native() -> Result<TargetMachine, String> {
        TargetMachine::new(default_triple())
    }

    /// Creates a target machine with the default options
    pub fn new(triple: String) -> Result<TargetMachine, String> {
        Ok(Target::from_triple(triple.clone())?.create_machine(triple))
    }

    /// Creates a target machine with the given options
    pub fn new_with_options(triple: String, cpu: String, features: String,
                            level: OptLevel) -> Result<TargetMachine, String> {
        Ok(Target::from_triple(triple.clone())?.create_machine_with_options(triple, cpu, features, level))
    }

    /// Emits code for a module to a given file with the given file type
    pub fn emit_module_to_file<P>(&self, module: &Module, file: P, file_type: FileType) -> Result<(), String>
        where P: AsRef<Path> {
        unsafe {
            LLVM_InitializeAllAsmPrinters();
            let file_str = into_c(file.as_ref().to_str().expect("invalid path")).into_raw();
            let mut error = null_mut();
            let flag = LLVMTargetMachineEmitToFile(
                self.machine,
                module.module.unwrap(),
                file_str,
                file_type.inner(),
                &mut error as *mut *mut i8,
            ) == 1;
            CString::from_raw(file_str);
            if flag {
                Err(from_c(error).unwrap_or(String::new()))
            } else {
                Ok(())
            }
        }
    }

    /// Creates a data layout based on this target machine
    pub fn data_layout(&self) -> TargetData {
        TargetData {
            data: unsafe {
                LLVMCreateTargetDataLayout(self.machine)
            }
        }
    }

    /// Gets the target of this target machine
    pub fn target(&self) -> Target {
        Target {
            target: unsafe{
                LLVMGetTargetMachineTarget(self.machine)
            }
        }
    }

    /// Gets the target triple of this target machine
    pub fn triple(&self) -> String {
        unsafe {
            from_c(LLVMGetTargetMachineTriple(self.machine)).unwrap_or(String::new())
        }
    }

    /// Gets the cpu of this target machine
    pub fn cpu(&self) -> String {
        unsafe {
            from_c(LLVMGetTargetMachineCPU(self.machine)).unwrap_or(String::new())
        }
    }

    /// Gets the features of this target machine
    pub fn features(&self) -> String {
        unsafe {
            from_c(LLVMGetTargetMachineFeatureString(self.machine)).unwrap_or(String::new())
        }
    }
}

impl Debug for TargetMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TargetMachine({:?}, {:?}, {:?}, {:?})", self.target(), self.triple(), self.cpu(), self.features())
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetMachine(self.machine);
        }
    }
}

/// A wrapper around a `LLVMTargetDataRef`
#[derive(Clone)]
pub struct TargetData {
    pub(crate) data: LLVMTargetDataRef,
}

impl TargetData {
    /// Returns the byte order of this data layout
    pub fn byte_order(&self) -> ByteOrdering {
        unsafe {
            use llvm_sys::target::LLVMByteOrdering::*;
            use self::ByteOrdering::*;
            match LLVMByteOrder(self.data) {
                LLVMBigEndian => BigEndian,
                LLVMLittleEndian => LittleEndian,
            }
        }
    }

    /// Returns the size of a pointer
    pub fn size_of_ptr(&self) -> u64 {
        unsafe {
            LLVMPointerSize(self.data) as u64
        }
    }

    /// Returns the size of a pointer in bits
    pub fn bit_size_of_ptr(&self) -> u64 {
        unsafe {
            LLVMPointerSize(self.data) as u64 * 8
        }
    }

    /// Returns the byte offset of an element in a struct
    pub fn offset_of_element(&self, ty: Type, index: u32) -> u64 {
        unsafe {
            LLVMOffsetOfElement(self.data, ty.ty, index)
        }
    }

    /// Returns the element at a byte offset in a struct
    pub fn element_at_offset(&self, ty: Type, offset: u64) -> u32 {
        unsafe {
            LLVMElementAtOffset(self.data, ty.ty, offset)
        }
    }

    /// Returns the size of a type
    pub fn size_of(&self, ty: Type) -> u64 {
        unsafe {
            LLVMABISizeOfType(self.data, ty.ty)
        }
    }

    /// Returns the size of a type in bits
    pub fn bit_size_of(&self, ty: Type) -> u64 {
        unsafe {
            LLVMSizeOfTypeInBits(self.data, ty.ty)
        }
    }

    /// Returns the size of a type when stored
    pub fn store_size_of(&self, ty: Type) -> u64 {
        unsafe {
            LLVMStoreSizeOfType(self.data, ty.ty)
        }
    }

    /// Returns the ABI alignment of a type
    pub fn abi_alignment_of(&self, ty: Type) -> u32 {
        unsafe {
            LLVMABIAlignmentOfType(self.data, ty.ty)
        }
    }

    /// Returns the ABI alignment of a type
    pub fn call_frame_alignment_of(&self, ty: Type) -> u32 {
        unsafe {
            LLVMCallFrameAlignmentOfType(self.data, ty.ty)
        }
    }

    /// Returns the preferred alignment of a type
    pub fn preferred_alignment_of(&self, ty: Type) -> u32 {
        unsafe {
            LLVMPreferredAlignmentOfType(self.data, ty.ty)
        }
    }
}

impl Debug for TargetData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TargetData({:?})", self.to_string())
    }
}

impl Display for TargetData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe {
            from_c(LLVMCopyStringRepOfTargetData(self.data)).unwrap_or(String::new())
        })
    }
}

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetData(self.data);
        }
    }
}