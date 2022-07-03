#![no_std]

#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        /// Kernel entry point.
        #[export_name = "_start"]
        pub extern "C" fn __kernel_start() -> ! {
            // validate the signature of the program entry point
            let f: fn() -> ! = $path;
            f()
        }
    };
}
