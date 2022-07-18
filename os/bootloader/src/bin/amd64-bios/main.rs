#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![cfg(target_arch = "x86_64")]
#![cfg(target_vendor = "unknown")]

mod panic;

use core::arch::global_asm;
use hw::{devices::serial::uart::UART, hal::serial::ByteSerial, UnsafeDefault};

global_asm!(include_str!("asm/stage1.s"));
global_asm!(include_str!("asm/stage2.s"));
global_asm!(include_str!("asm/stage3.s"));
global_asm!(include_str!("asm/stage4.s"));
global_asm!(include_str!("asm/vga.s"));
global_asm!(include_str!("asm/gdt_defs.s"));

static mut UART: Option<UART> = None;

pub unsafe fn print(str: &str) {
    if let Some(uart) = &UART {
        uart.puts(str)
    }
}

pub unsafe fn println(str: &str) {
    print(str);
    print("\n\r");
}

#[no_mangle]
unsafe extern "C" fn stage5() -> ! {
    UART = Some(UART::default());
    main();
    println("halting execution!");
    loop {}
}

unsafe fn main() {
    println("hello world!");
}
