//
//! Binary example for no-std with allocation.
//
// required features: no-std,alloc,libc
// required profile: no-std
//
// run with:
// cargo run --bin alloc --no-default-features --features=no-std,alloc,libc --profile=no-std

#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use revela::{
    backend::{prompt, ALLOCATOR},
};
use libc_print::{libc_println as println};

/* app */

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    /* init the custom global allocator */

    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe {
        ALLOCATOR.lock().init_from_slice(&mut HEAP_MEM);
    }

    /* the program itself */

    println!("You entered: {}", prompt("Enter some text: "));

    0
}
