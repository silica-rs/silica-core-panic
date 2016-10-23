#![feature(lang_items)]
#![feature(panic_runtime)]
#![feature(unwind_attributes)]
#![feature(asm)]

#![no_std]
#![panic_runtime]

#![allow(private_no_mangle_fns)]

/// Calls during unwinding stages
/// 1. search for a "catch"
#[cfg(target_arch = "arm")]
#[lang = "eh_personality"]
#[no_mangle]
unsafe extern "C" fn rust_eh_personality() {
    asm!("bkpt")
}

/// This function is needed to resume unwinding
/// at the end of landing pads
/// enabled by the target configuration `custom-unwind-resume`
#[cfg(target_arch = "arm")]
#[lang = "eh_unwind_resume"]
#[unwind]
unsafe extern "C" fn rust_eh_unwind_resume(ptr: *mut u8) -> ! {
    let _ = ptr; // kill compiler warning for unused variable

    asm!("bkpt");
    loop {}
}

/// Equivalent to c++'s throw
#[cfg(target_arch = "arm")]
#[lang = "panic_fmt"]
#[unwind]
pub extern fn panic_impl(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { asm!("bkpt") }
    loop {}
}
