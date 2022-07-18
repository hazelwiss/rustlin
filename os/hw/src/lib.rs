#![no_std]
#![feature(let_chains)]

pub mod arch;
pub mod devices;
pub mod hal;
pub mod mem;

pub unsafe trait UnsafeDefault {
    unsafe fn default() -> Self;
}
