#![feature(lang_items, start)]

mod consts;
mod types;

use consts::*;
use types::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn DllRegisterServer() -> HResult {
    0
}

// ICorProfilerCallback

#[no_mangle]
pub unsafe extern "C" fn AddRef(ptr: ThisPtr) -> Ulong {
    unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn Release(ptr: ThisPtr) -> HResult {
    unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn Shutdown(ptr: ThisPtr) -> HResult {
    unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn Initialize(ptr: ThisPtr, pICorProfilerInfoUnk: IUnknownPtr) -> HResult {
    unimplemented!()
}
// ICorProfilerCallback * This,
// /* [in] */  {

#[no_mangle]
pub unsafe extern "C" fn AppDomainCreationStarted(
    ptr: ThisPtr,
    appDomainId: AppDomainID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
// /* [in] */ AppDomainID appDomainId);

#[no_mangle]
pub unsafe extern "C" fn AppDomainCreationFinished(
    ptr: ThisPtr,
    appDomainID: AppDomainID,
    hrStatus: HResult,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AppDomainID appDomainId,
// /* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn AppDomainShutdownStarted(
    ptr: ThisPtr,
    appDomainID: AppDomainID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AppDomainID appDomainId);

#[no_mangle]
pub unsafe extern "C" fn AppDomainShutdownFinished(
    ptr: ThisPtr,
    appDomainID: AppDomainID,
    hrStatus: HResult,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AppDomainID appDomainId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn AssemblyLoadStarted(ptr: ThisPtr, assemblyId: AssemblyID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AssemblyID assemblyId);

#[no_mangle]
pub unsafe extern "C" fn AssemblyLoadFinished(ptr: ThisPtr, assemblyID: AssemblyID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AssemblyID assemblyId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn AssemblyUnloadStarted(ptr: ThisPtr, assemblyID: AssemblyID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AssemblyID assemblyId);

#[no_mangle]
pub unsafe extern "C" fn AssemblyUnloadFinished(
    ptr: ThisPtr,
    assemblyID: AssemblyID,
    hrStatus: HResult,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ AssemblyID assemblyId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn ModuleLoadStarted(ptr: ThisPtr, moduleId: ModuleID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ModuleID moduleId);

#[no_mangle]
pub unsafe extern "C" fn ModuleLoadFinished(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ModuleID moduleId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn ModuleUnloadStarted(ptr: ThisPtr, moduleID: ModuleID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ModuleID moduleId);

#[no_mangle]
pub unsafe extern "C" fn ModuleUnloadFinished(
    ptr: ThisPtr,
    moduleID: ModuleID,
    hrStatus: HResult,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ModuleID moduleId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn ModuleAttachedToAssembly(
    ptr: ThisPtr,
    moduleID: ModuleID,
    assemblyID: AssemblyID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ModuleID moduleId,
///* [in] */ AssemblyID AssemblyId);

#[no_mangle]
pub unsafe extern "C" fn ClassLoadStarted(ptr: ThisPtr, classID: ClassID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ClassID classId);

#[no_mangle]
pub unsafe extern "C" fn ClassLoadFinished(
    ptr: ThisPtr,
    classID: ClassID,
    hrStatus: HResult,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ClassID classId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn ClassUnloadStarted(ptr: ThisPtr, classID: ClassID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ClassID classId);

#[no_mangle]
pub unsafe extern "C" fn ClassUnloadFinished(
    ptr: ThisPtr,
    classID: ClassID,
    hrStatus: HResult,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ClassID classId,
///* [in] */ HResult hrStatus);

#[no_mangle]
pub unsafe extern "C" fn FunctionUnloadStarted(ptr: ThisPtr, functionID: FunctionID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn JITCompilationStarted(
    ptr: ThisPtr,
    functionId: FunctionID,
    fIsSafeToBlock: Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [in] */ Bool fIsSafeToBlock);

#[no_mangle]
pub unsafe extern "C" fn JITCompilationFinished(
    ptr: ThisPtr,
    functionId: FunctionID,
    hrStatus: HResult,
    fIsSafeToBlock: Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [in] */ HResult hrStatus,
///* [in] */ Bool fIsSafeToBlock);

#[no_mangle]
pub unsafe extern "C" fn JITCachedFunctionSearchStarted(
    ptr: ThisPtr,
    functionId: FunctionID,
    pbUseCachedFunction: *mut Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [out] */ Bool *pbUseCachedFunction);

#[no_mangle]
pub unsafe extern "C" fn JITCachedFunctionSearchFinished(
    ptr: ThisPtr,
    functionId: FunctionID,
    result: CorPrfJITCache,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [in] */ CorPrfJITCache result);

#[no_mangle]
pub unsafe extern "C" fn JITFunctionPitched(ptr: ThisPtr, functionId: FunctionID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn JITInlining(
    ptr: ThisPtr,
    calledId: FunctionID,
    calleeId: FunctionID,
    pfShouldInline: *mut Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID callerId,
///* [in] */ FunctionID calleeId,
///* [out] */ Bool *pfShouldInline);

#[no_mangle]
pub unsafe extern "C" fn ThreadCreated(ptr: ThisPtr, threadId: ThreadID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ThreadID threadId);

#[no_mangle]
pub unsafe extern "C" fn ThreadDestroyed(ptr: ThisPtr, threadID: ThreadID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ThreadID threadId);

#[no_mangle]
pub unsafe extern "C" fn ThreadAssignedToOSThread(
    ptr: ThisPtr,
    managedThreadID: ThreadID,
    osThreadId: DWord,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ThreadID managedThreadId,
///* [in] */ DWORD osThreadId);

#[no_mangle]
pub unsafe extern "C" fn RemotingClientInvocationStarted(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RemotingClientSendingMessage(
    ptr: ThisPtr,
    pCookie: *mut Guid,
    fIsAsync: Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ GUID *pCookie,
///* [in] */ Bool fIsAsync);

#[no_mangle]
pub unsafe extern "C" fn RemotingClientReceivingReply(
    ptr: ThisPtr,
    pCookie: *mut Guid,
    fIsAsync: Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ GUID *pCookie,
///* [in] */ Bool fIsAsync);

#[no_mangle]
pub unsafe extern "C" fn RemotingClientInvocationFinished(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RemotingServerReceivingMessage(
    ptr: ThisPtr,
    pCookie: *mut Guid,
    fIsAsync: Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ GUID *pCookie,
///* [in] */ Bool fIsAsync);

#[no_mangle]
pub unsafe extern "C" fn RemotingServerInvocationStarted(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RemotingServerInvocationReturned(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RemotingServerSendingReply(
    ptr: ThisPtr,
    pCookie: *mut Guid,
    fIsAsync: Bool,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ GUID *pCookie,
///* [in] */ Bool fIsAsync);

#[no_mangle]
pub unsafe extern "C" fn UnmanagedToManagedTransition(
    ptr: ThisPtr,
    functionId: FunctionID,
    reason: CorPrfTransitionReason,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [in] */ CorPrfTransitionReason reason);

#[no_mangle]
pub unsafe extern "C" fn ManagedToUnmanagedTransition(
    ptr: ThisPtr,
    functionId: FunctionID,
    reason: CorPrfTransitionReason,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [in] */ CorPrfTransitionReason reason);

#[no_mangle]
pub unsafe extern "C" fn RuntimeSuspendStarted(
    ptr: ThisPtr,
    suspendReason: CorPrfSuspendReason,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ CorPrfSuspendReason suspendReason);

#[no_mangle]
pub unsafe extern "C" fn RuntimeSuspendFinished(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RuntimeSuspendAborted(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RuntimeResumeStarted(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RuntimeResumeFinished(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn RuntimeThreadSuspended(ptr: ThisPtr, threadId: ThreadID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ThreadID threadId);

#[no_mangle]
pub unsafe extern "C" fn RuntimeThreadResumed(ptr: ThisPtr, threadId: ThreadID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ThreadID threadId);

#[no_mangle]
pub unsafe extern "C" fn MovedReferences(
    ptr: ThisPtr,
    cMovedObjectIDRanges: Ulong,
    oldObjectIDRangeStart: *mut ObjectID,
    newObjectIDRangeStart: *mut ObjectID,
    cObjectIDRangeLength: Ulong,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ Ulong cMovedObjectIDRanges,
///* [size_is][in] */ ObjectID oldObjectIDRangeStart[  ],
///* [size_is][in] */ ObjectID newObjectIDRangeStart[  ],
///* [size_is][in] */ Ulong cObjectIDRangeLength[  ]);

#[no_mangle]
pub unsafe extern "C" fn ObjectAllocated(
    ptr: ThisPtr,
    objectId: ObjectID,
    classID: ClassID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ObjectID objectId,
///* [in] */ ClassID classId);

#[no_mangle]
pub unsafe extern "C" fn ObjectsAllocatedByClass(
    ptr: ThisPtr,
    classIds: *mut ClassID,
    cObjects: *mut Ulong,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ Ulong cClassCount,
///* [size_is][in] */ ClassID classIds[  ],
///* [size_is][in] */ Ulong cObjects[  ]);

#[no_mangle]
pub unsafe extern "C" fn ObjectReferences(
    ptr: ThisPtr,
    objectId: ObjectID,
    classId: ClassID,
    cObjectRefs: Ulong,
    objectIdRefIds: *mut ObjectID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ObjectID objectId,
///* [in] */ ClassID classId,
///* [in] */ Ulong cObjectRefs,
///* [size_is][in] */ ObjectID objectRefIds[  ]);

#[no_mangle]
pub unsafe extern "C" fn RootReferences(
    ptr: ThisPtr,
    cRootRefs: Ulong,
    rootRefIds: *mut ObjectID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ Ulong cRootRefs,
///* [size_is][in] */ ObjectID rootRefIds[  ]);

#[no_mangle]
pub unsafe extern "C" fn ExceptionThrown(ptr: ThisPtr, thrownObjectId: ObjectID) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ObjectID thrownObjectId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionSearchFunctionEnter(
    ptr: ThisPtr,
    functionId: FunctionID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionSearchFunctionLeave(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn ExceptionSearchFilterEnter(
    ptr: ThisPtr,
    functionId: FunctionID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionSearchFilterLeave(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn ExceptionSearchCatcherFound(
    ptr: ThisPtr,
    functionId: FunctionID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionOSHandlerEnter(ptr: ThisPtr, _unused: *mut u32) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ UINT_PTR __unused);

#[no_mangle]
pub unsafe extern "C" fn ExceptionOSHandlerLeave(ptr: ThisPtr, _unused: *mut u32) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ UINT_PTR __unused);

#[no_mangle]
pub unsafe extern "C" fn ExceptionUnwindFunctionEnter(
    ptr: ThisPtr,
    functionId: FunctionID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionUnwindFunctionLeave(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn ExceptionUnwindFinallyEnter(
    ptr: ThisPtr,
    functionId: FunctionID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionUnwindFinallyLeave(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn ExceptionCatcherEnter(
    ptr: ThisPtr,
    functionId: FunctionID,
    objectId: ObjectID,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ FunctionID functionId,
///* [in] */ ObjectID objectId);

#[no_mangle]
pub unsafe extern "C" fn ExceptionCatcherLeave(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn COMClassicVTableCreated(
    ptr: ThisPtr,
    classId: ClassID,
    implementedIID: RefGuid,
    pVTable: *mut u8,
    cSlots: Ulong,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ClassID wrappedClassId,
///* [in] */ RefGuid implementedIID,
///* [in] */ void *pVTable,
///* [in] */ Ulong cSlots);

#[no_mangle]
pub unsafe extern "C" fn COMClassicVTableDestroyed(
    ptr: ThisPtr,
    wrappedCLassId: ClassID,
    implementedIID: RefGuid,
    pVTable: *mut u8,
) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This,
///* [in] */ ClassID wrappedClassId,
///* [in] */ RefGuid implementedIID,
///* [in] */ void *pVTable);

#[no_mangle]
pub unsafe extern "C" fn ExceptionCLRCatcherFound(ptr: ThisPtr) -> HResult {
    unimplemented!()
}
//ICorProfilerCallback * This);

#[no_mangle]
pub unsafe extern "C" fn ExceptionCLRCatcherExecute(ptr: ThisPtr) -> HResult {
    unimplemented!()
}


// ICorProfilerCallback2
pub unsafe extern "C" fn ThreadNameChanged( 
            /* [in] */ threadId: ThreadID ,
            /* [in] */ cchName: Ulong,
            /* [annotation][in] */ 
            name: *mut u16) -> HResult {
    unimplemented!()
}

/*pub unsafe extern "C" fn GarbageCollectionStarted( 
            /* [in] */ cGenerations: usize,
            /* [size_is][in] */ generationCollected: *mut Bool,
            /* [in] */ reason: CorPrfGcReason) {
    unimplemented!()
}*/
        
pub unsafe extern "C" fn GarbageCollectionStarted( 
            /* [in] */ cGenerations: usize ,
            /* [size_is][in] */ generationCollected: *mut Bool,
            /* [in] */ reason: CorPrfGcReason)
{
    unimplemented!()
}

pub unsafe extern "C" fn SurvivingReferences( 
            /* [in] */ cSurvivingObjectIDRanges: Ulong,
            /* [size_is][in] */ objectIDRangeStart: *mut ObjectID,
            /* [size_is][in] */ cObjectIDRangeLength: Ulong) {
    unimplemented!()
}
        
pub unsafe extern "C" fn GarbageCollectionFinished() {
    unimplemented!()
}
        
pub unsafe extern "C" fn FinalizeableObjectQueued( 
            /* [in] */ finalizerFlags: DWord,
            /* [in] */ objectID: ObjectID ) {
    unimplemented!()
}
        
pub unsafe extern "C" fn RootReferences2( 
            /* [in] */ cRootRefs: Ulong,
            /* [size_is][in] */ rootRefIds: *mut ObjectID,
            /* [size_is][in] */ rootKinds: *mut CorPrfGcRootKind,
            /* [size_is][in] */ rootFlags: *mut CorPrfGcRootFlags,
            /* [size_is][in] */ rootIds: UIntPtr) {
    unimplemented!()
}

pub unsafe extern "C" fn HandleCreated(
            /* [in] */ handleId: GCHandleID ,
            /* [in] */ initialObjectId: ObjectID) -> HResult {
    unimplemented!()
}

pub unsafe extern "C" fn HandleDestroyed(
    /* [in] */ handleId: GCHandleID ) {
    unimplemented!()
}
