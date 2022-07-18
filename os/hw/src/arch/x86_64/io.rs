pub mod byte {
    use core::arch::asm;

    pub unsafe fn io_out(port: u16, val: u8) {
        asm! {
            "out dx, al",
            in("al") val,
            in("dx") port,
        };
    }

    pub unsafe fn io_in(port: u16) -> u8 {
        let mut out: i8;
        asm! {
            "in al, dx",
            out("al") out,
            in("dx") port,
        };
        out as u8
    }
}

pub mod word {
    use core::arch::asm;

    pub unsafe fn io_out(port: u16, val: u16) {
        asm! {
            "out dx, ax",
            in("ax") val,
            in("dx") port,
        };
    }

    pub unsafe fn io_in(port: u16) -> u16 {
        let mut out: i32;
        asm! {
            "in ax, dx",
            out("ax") out,
            in("dx") port,
        };
        out as u16
    }
}
