#![no_std]
#![no_main]

mod panic;

#[no_mangle]
extern "C" fn _start() -> ! {
    let hello = "hello I am wise man am I now";
    unsafe {
        *(0xFFFF as *mut u32) = &hello as *const _ as u32;
    }
    loop {}
}
