#![feature(lang_items, start)]

mod consts;

use consts::*;

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

#[no_mangle]
pub unsafe extern "C" fn AppDomainCreationFinished(appDomainId: AppDomainID, hrStatus: HResult) -> HResult {
    println!("App domain creation was finished!");
	0
}

/*#[no_mangle]
pub extern "C" fn _DllMainCRTStartup() {

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	loop{}
}


#[lang = "eh_personality"] extern fn eh_personality() {}*/
