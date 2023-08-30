// revela::backend::alloc
//
//! utilities for `alloc`.
//

use linked_list_allocator::LockedHeap;

/// A simple global allocator using a locked heap.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "alloc", feature = "no_std")))
)]
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(all(feature = "libc", feature = "unsafe_libc", feature = "alloc"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "libc", feature = "unsafe_libc", feature = "alloc")))
)]
pub use utils_libc::*;

#[cfg(feature = "libc")]
// MAYBE?
// doc(cfg(all(feature = "alloc", not(feature = "std"))))
mod utils_libc {
    use alloc::{ffi::CString, string::String, vec::Vec};
    use core::ffi::CStr;
    use libc_print::libc_print as print;

    /// Prompts the user for an input.
    #[cfg(all(feature = "libc", feature = "unsafe_libc"))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(feature = "libc", feature = "unsafe_libc")))
    )]
    #[inline]
    pub fn prompt(text: &str) -> String {
        print!("{text}");

        // bin_size: 26_668
        read_string_buf::<255>()

        // bin_size: 30_784
        // read_string()
    }

    /// Reads a string with a maximum `BUF_LEN` -1 number of bytes.
    #[cfg(all(feature = "libc", feature = "unsafe_libc"))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(feature = "libc", feature = "unsafe_libc")))
    )]
    pub fn read_string_buf<const BUF_LEN: usize>() -> String {
        let mut buf = [0_u8; BUF_LEN];
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

            // Always need a trailing 0.
            if i == buf.len() - 1 {
                break;
            }
        }
        let cstr = CStr::from_bytes_with_nul(&buf[..i + 1]).expect("invalid cstring");
        cstr.to_string_lossy().into()
    }

    /// Reads a string.
    //
    // More convenient, more binary size (about 4KiB).
    #[cfg(all(feature = "libc", feature = "unsafe_libc"))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(all(feature = "libc", feature = "unsafe_libc")))
    )]
    pub fn read_string() -> String {
        let mut buf = Vec::<u8>::new();
        loop {
            let c: i32;
            unsafe {
                c = libc::getchar();
            }
            if c == b'\n' as i32 || c == libc::EOF {
                break;
            }
            buf.push(c as u8);
        }
        buf.push(0);
        CString::from_vec_with_nul(buf)
            .expect("invalid input")
            .into_string()
            .expect("invalid string")
    }
}
