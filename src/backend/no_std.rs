// revela::backend::no_std
//
//! Utilities for `no_std`.
//

/// A panic handler that prints the panic info and exits with exit error 1.
#[panic_handler]
#[cfg(all(feature = "libc", feature = "unsafe_libc"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "libc", feature = "unsafe_libc")))
)]
pub fn panic_exit(info: &core::panic::PanicInfo) -> ! {
    libc_print::libc_println!("Panic!!!\n{info:#?}\nExiting. . .");
    unsafe {
        libc::exit(1);
    }
}

/// A panic handler that gives no information and just loops indefinitely.
#[panic_handler]
#[cfg(not(all(feature = "libc", feature = "unsafe_libc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(not(all(feature = "libc", feature = "unsafe_libc"))))
)]
pub fn panic_loop(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Returns a single char.
#[cfg(all(feature = "libc", feature = "unsafe_libc"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "libc", feature = "unsafe_libc")))
)]
pub fn read_char() -> char {
    let c: i32;
    unsafe {
        c = libc::getchar();
    }
    char::from_u32(c as u32).expect("invalid char")
}
