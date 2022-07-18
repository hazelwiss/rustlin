#[panic_handler]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Some(message) = info.message() {
        if let Some(str) = message.as_str() {
            crate::print(str)
        }
    }
    loop {}
}
