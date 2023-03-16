//
//! Binary example for no-std without allocation.
//
// required features: no-std,libc
// required profile: no-std
//
// run with:
// cargo run --example no_alloc --no-default-features --features=no-std,libc --profile=no-std

#![no_std]
#![no_main]

use core::ptr;

#[allow(unused)]
use revela::backend::panic_exit;

// multiple char
#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    const PROMPT: &str = "Enter a line of text:\n\0";

    unsafe {
        libc::printf(PROMPT.as_ptr() as *const _);
    }

    let mut buf = [0u8; 1024];
    let mut i = 0;

    loop {
        let c: i32;
        unsafe {
            c = libc::getchar();
        }

        if c == b'\n' as i32 || c == libc::EOF {
            break;
        }

        buf[i] = c as u8;
        i += 1;

        if i == buf.len() {
            // Buffer is full, so stop reading more characters.
            break;
        }
    }

    unsafe {
        libc::printf("You entered: \0".as_ptr() as *const _);
        libc::printf(ptr::null()); // Print a null-terminated string.
    }

    // let _len = i as usize;
    unsafe {
        // We know that buf contains a null-terminated string.
        libc::puts(buf.as_ptr() as *const _);
    }

    0
}
