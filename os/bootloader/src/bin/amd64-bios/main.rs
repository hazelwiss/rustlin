#![no_std]
#![no_main]
#![cfg(target_arch = "x86_64")]
#![cfg(target_vendor = "unknown")]

mod panic;

use core::arch::global_asm;

global_asm!(include_str!("asm/stage1.s"));

#[no_mangle]
extern "C" fn stage4() -> ! {
    let hello = "hello I am wise man am I now";
    unsafe {
        *(0xFFF2 as *mut u32) = &hello as *const _ as u32;
    }
    loop {}
}
