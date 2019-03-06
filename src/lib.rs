#![feature(lang_items, start)]

mod consts;

use consts::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn test() {
    // nothing

}

pub fn AppDomainCreationFinished(appDomainId: AppDomainID, hrStatus: HResult) -> HResult {
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
