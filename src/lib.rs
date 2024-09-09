#![no_std]

use libc_print::libc_println;

#[cfg_attr(target_os = "linux", link_section = ".init_array")]
pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_initialise_fn;

extern "C" fn ld_preload_initialise_fn() {
    // Did some test with a panicking function.
    // panic!("LET'S FAIL!");

    // Just printing something
    libc_println!("HOLA!");
}
