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

#[no_mangle]
pub extern "C" fn test() {
    // nothing

}

#[no_mangle]
pub unsafe  extern "C" fn Initialize(pICorProfilerInfoUnk: *mut u8) -> HResult {
    println!("Initialized!");
    0
}

/*#[no_mangle]
pub unsafe extern "C" fn AppDomainCreationFinished(appDomainId: AppDomainID, hrStatus: HResult) -> HResult {
    println!("App domain creation was finished!");
	0
}*/

type ThisPtr = *mut u8;
type Ulong = u64;

		pub unsafe extern "C" fn AddRef(ptr: ThisPtr) -> ULONG {
			unimplemented!()
		}

		pub unsafe extern "C" fn Release(ptr: ThisPtr) -> Hresult {
			unimplemented!()
		}
        
        pub unsafe extern "C" fn Shutdown(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
        
        //pub unsafe extern "C" fn Initialize(ptr: ThisPtr, pICorProfilerInfoUnk: IUnknownPtr) -> HResult ( 
	//		unimplemented!()
//		}
            // ICorProfilerCallback * This,
            // /* [in] */  {
        
        pub unsafe extern "C" fn AppDomainCreationStarted(ptr: ThisPtr, appDomainId: AppDomainID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            // /* [in] */ AppDomainID appDomainId);
        
        pub unsafe extern "C" fn AppDomainCreationFinished(ptr: ThisPtr, appDomainID: AppDomainID, hrStatus: HResult) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AppDomainID appDomainId,
            // /* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn AppDomainShutdownStarted(ptr: ThisPtr, appDomainID: AppDomainID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AppDomainID appDomainId);
        
        pub unsafe extern "C" fn AppDomainShutdownFinished(ptr: ThisPtr, appDomainID: AppDomainID, hrStatus: HResult) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AppDomainID appDomainId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn AssemblyLoadStarted(ptr: ThisPtr, assemblyId: AssemblyID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AssemblyID assemblyId);
        
        pub unsafe extern "C" fn AssemblyLoadFinished(ptr: ThisPtr, assemblyID: AssemblyID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AssemblyID assemblyId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn AssemblyUnloadStarted(ptr: ThisPtr, assemblyID: AssemblyID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AssemblyID assemblyId);
        
        pub unsafe extern "C" fn AssemblyUnloadFinished(ptr: ThisPtr, assemblyID: AssemblyID, hrStatus: HResult) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ AssemblyID assemblyId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn ModuleLoadStarted(ptr: ThisPtr, moduleId: ModuleID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ModuleID moduleId);
        
        pub unsafe extern "C" fn ModuleLoadFinished(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ModuleID moduleId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn ModuleUnloadStarted(ptr: ThisPtr, moduleID: ModuleID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ModuleID moduleId);
        
        pub unsafe extern "C" fn ModuleUnloadFinished(ptr: ThisPtr, moduleID: ModuleID, hrStatus: HResult) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ModuleID moduleId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn ModuleAttachedToAssembly(ptr: ThisPtr, moduleID: ModuleID, assemblyID: AssemblyID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ModuleID moduleId,
            ///* [in] */ AssemblyID AssemblyId);
        
        pub unsafe extern "C" fn ClassLoadStarted(ptr: ThisPtr, classID: ClassID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ClassID classId);
        
        pub unsafe extern "C" fn ClassLoadFinished(ptr: ThisPtr, classID: ClassID, hrStatus: HResult) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ClassID classId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn ClassUnloadStarted(ptr: ThisPtr, classID: ClassID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ClassID classId);
        
        pub unsafe extern "C" fn ClassUnloadFinished(ptr: ThisPtr, classID: ClassID, hrStatus: HResult) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ClassID classId,
            ///* [in] */ HResult hrStatus);
        
        pub unsafe extern "C" fn FunctionUnloadStarted(ptr: ThisPtr, functionID: FunctionID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn JITCompilationStarted(ptr: ThisPtr, functionId: FunctionID, fIsSafeToBlock: BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [in] */ BOOL fIsSafeToBlock);
        
        pub unsafe extern "C" fn JITCompilationFinished(ptr: ThisPtr, functionId: FunctionID, hrStatus: HResult, fIsSafeToBlock: BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [in] */ HResult hrStatus,
            ///* [in] */ BOOL fIsSafeToBlock);
        
        pub unsafe extern "C" fn JITCachedFunctionSearchStarted(ptr: ThisPtr, functionId: FunctionID, mut* BOOL pbUseCachedFunction) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [out] */ BOOL *pbUseCachedFunction);
        
        pub unsafe extern "C" fn JITCachedFunctionSearchFinished(ptr: ThisPtr, functionId: FunctionID, result: COR_PRF_JIT_CACHE) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [in] */ COR_PRF_JIT_CACHE result);
        
        pub unsafe extern "C" fn JITFunctionPitched(ptr: ThisPtr, functionId: FunctionId) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn JITInlining(ptr: ThisPtr, calledId: FunctionId, calleeId: FunctionId, pfShouldInline: mut* BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID callerId,
            ///* [in] */ FunctionID calleeId,
            ///* [out] */ BOOL *pfShouldInline);
        
        pub unsafe extern "C" fn ThreadCreated(ptr: ThisPtr, threadId: ThreadID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ThreadID threadId);
        
        pub unsafe extern "C" fn ThreadDestroyed(ptr: ThisPtr, threadID: ThreadID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ThreadID threadId);
        
        pub unsafe extern "C" fn ThreadAssignedToOSThread(ptr: ThisPtr, managedThreadID: ThreadID, DWORD osThreadId) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ThreadID managedThreadId,
            ///* [in] */ DWORD osThreadId);
        
        pub unsafe extern "C" fn RemotingClientInvocationStarted(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RemotingClientSendingMessage(ptr: ThisPtr, pCookie: mut* Guid, fIsAsync: BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ GUID *pCookie,
            ///* [in] */ BOOL fIsAsync);
        
        pub unsafe extern "C" fn RemotingClientReceivingReply(ptr: ThisPtr, pCookie: mut* Guid, fIsAsync: BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ GUID *pCookie,
            ///* [in] */ BOOL fIsAsync);
        
        pub unsafe extern "C" fn RemotingClientInvocationFinished(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RemotingServerReceivingMessage(ptr: ThisPtr, pCookie: mut* Guid, fIsAsync: BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ GUID *pCookie,
            ///* [in] */ BOOL fIsAsync);
        
        pub unsafe extern "C" fn RemotingServerInvocationStarted(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RemotingServerInvocationReturned(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RemotingServerSendingReply(ptr: ThisPtr, pCookie: mut* Guid, fIsAsync: BOOL) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ GUID *pCookie,
            ///* [in] */ BOOL fIsAsync);
        
        pub unsafe extern "C" fn UnmanagedToManagedTransition(ptr: ThisPtr, FunctionID functionId, reason: COR_PRF_TRANSITION_REASON ) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [in] */ COR_PRF_TRANSITION_REASON reason);
        
        pub unsafe extern "C" fn ManagedToUnmanagedTransition(ptr: ThisPtr, functionId: FunctionID, reason: COR_PRF_TRANSITION_REASON) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [in] */ COR_PRF_TRANSITION_REASON reason);
        
        pub unsafe extern "C" fn RuntimeSuspendStarted(ptr: ThisPtr, suspendReason: COR_PRF_SUSPEND_REASON) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ COR_PRF_SUSPEND_REASON suspendReason);
        
        pub unsafe extern "C" fn RuntimeSuspendFinished(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RuntimeSuspendAborted(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RuntimeResumeStarted(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RuntimeResumeFinished(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn RuntimeThreadSuspended(ptr: ThisPtr, threadId: ThreadID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ThreadID threadId);
        
        pub unsafe extern "C" fn RuntimeThreadResumed(ptr: ThisPtr, threadId: ThreadID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ThreadID threadId);
        
        pub unsafe extern "C" fn MovedReferences(ptr: ThisPtr, cMovedObjectIDRanges: ULONG, oldObjectIDRangeStart: *mut ObjectID, newObjectIDRangeStart: *mut ObjectID, cObjectIDRangeLength: ULONG) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ULONG cMovedObjectIDRanges,
            ///* [size_is][in] */ ObjectID oldObjectIDRangeStart[  ],
            ///* [size_is][in] */ ObjectID newObjectIDRangeStart[  ],
            ///* [size_is][in] */ ULONG cObjectIDRangeLength[  ]);
        
        pub unsafe extern "C" fn ObjectAllocated(ptr: ThisPtr, objectId: ObjectID, classID: ClassID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ObjectID objectId,
            ///* [in] */ ClassID classId);
        
        pub unsafe extern "C" fn ObjectsAllocatedByClass(ptr: ThisPtr, classIds: *mut ClassID, cObjects: *mut ULONG) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ULONG cClassCount,
            ///* [size_is][in] */ ClassID classIds[  ],
            ///* [size_is][in] */ ULONG cObjects[  ]);
        
        pub unsafe extern "C" fn ObjectReferences(ptr: ThisPtr, objectId: ObjectID, classId: ClassID, cObjectRefs: ULONG, objectIdRefIds: *mut ObjectID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ObjectID objectId,
            ///* [in] */ ClassID classId,
            ///* [in] */ ULONG cObjectRefs,
            ///* [size_is][in] */ ObjectID objectRefIds[  ]);
        
        pub unsafe extern "C" fn RootReferences(ptr: ThisPtr, cRootRefs: ULONG, rootRefIds: *mut ObjectID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ULONG cRootRefs,
            ///* [size_is][in] */ ObjectID rootRefIds[  ]);
        
        pub unsafe extern "C" fn ExceptionThrown(ptr: ThisPtr, thrownObjectId: ObjectID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ObjectID thrownObjectId);
        
        pub unsafe extern "C" fn ExceptionSearchFunctionEnter(ptr: ThisPtr, functionId: FunctionID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn ExceptionSearchFunctionLeave(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn ExceptionSearchFilterEnter(ptr: ThisPtr, functionId: FunctionID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn ExceptionSearchFilterLeave(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn ExceptionSearchCatcherFound(ptr: ThisPtr, functionId: FunctionID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn ExceptionOSHandlerEnter(ptr: ThisPtr, _unused: *mut u32) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ UINT_PTR __unused);
        
        pub unsafe extern "C" fn ExceptionOSHandlerLeave(ptr: ThisPtr, _unused: *mut u32) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ UINT_PTR __unused);
        
        pub unsafe extern "C" fn ExceptionUnwindFunctionEnter(ptr: ThisPtr, functionId: FunctionID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn ExceptionUnwindFunctionLeave(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn ExceptionUnwindFinallyEnter(ptr: ThisPtr, functionId: FunctionID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId);
        
        pub unsafe extern "C" fn ExceptionUnwindFinallyLeave(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn ExceptionCatcherEnter(ptr: ThisPtr, functionId: FunctionID, objectId: ObjectID) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ FunctionID functionId,
            ///* [in] */ ObjectID objectId);
        
        pub unsafe extern "C" fn ExceptionCatcherLeave(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn COMClassicVTableCreated(ptr: ThisPtr, classId: ClassID, implementedIID: REFGUID, pVTable: *mut u8, cSlots: ULONG) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ClassID wrappedClassId,
            ///* [in] */ REFGUID implementedIID,
            ///* [in] */ void *pVTable,
            ///* [in] */ ULONG cSlots);
        
        pub unsafe extern "C" fn COMClassicVTableDestroyed(ptr: ThisPtr, wrappedCLassId: ClassID, implementedIID: ClassID, implementedIID: REFGUID, pVTable: *mut u8) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This,
            ///* [in] */ ClassID wrappedClassId,
            ///* [in] */ REFGUID implementedIID,
            ///* [in] */ void *pVTable);
        
        pub unsafe extern "C" fn ExceptionCLRCatcherFound(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
            //ICorProfilerCallback * This);
        
        pub unsafe extern "C" fn ExceptionCLRCatcherExecute(ptr: ThisPtr) -> HResult {
			unimplemented!()
		}
