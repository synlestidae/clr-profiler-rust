
pub type AssemblyID = UIntPtr;
pub type Bool = i32;
pub type ClassID = UIntPtr;
pub type CorPrfJITCache = i32;
pub type CorPrfSuspendReason = i32;
pub type CorPrfTransitionReason = i32;
pub type DWord = usize;
pub type FunctionID = UIntPtr;
pub type Guid = [u8; 16];
pub type MethodID = UIntPtr;
pub type ModuleID = UIntPtr;
pub type ObjectID = UIntPtr;
pub type RefGuid = *mut Guid;
pub type ThreadID = UIntPtr;
pub type UIntPtr = usize;
pub type Ulong = u64;
pub type ThisPtr = *mut usize;

