#![no_std]
#![no_main]

use core::{
    mem::zeroed,
    panic::PanicInfo,
    ptr::{read, write_volatile},
};

use core::arch::asm;

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: fn() -> ! = reset_handler;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On a panic, loop forever
    loop {}
}

/// NRF52 Specific reset handler tasks
pub fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut __sbss: u32; // Start of .bss section
        static mut __ebss: u32; // End of .bss section
        static mut __sdata: u32; // Start of .data section
        static mut __edata: u32; // End of .data section
        static __sidata: u32; // Start of .rodata section
    }

    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = &mut __sdata;
        let edata: *mut u32 = &mut __edata;
        let mut sidata: *const u32 = &__sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    // Jump to main
    main()
}

// ----------------------------------------------------------------
// CXX INTEGRATION
// ----------------------------------------------------------------
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn hook_a() -> u32;
    }
}

fn hook_a() -> u32 {
    0
}

// ----------------------------------------------------------------

// ----------------------------------------------------------------
// Application Level
// ----------------------------------------------------------------

/// GPIO MMIO REGS:
const REG_P0_PIN_CNF_BASE: *mut u32 = 0x5000_0700 as *mut u32;
const REG_P0_OUT_SET: *mut u32 = 0x5000_0508 as *mut u32;
const REG_P0_OUT_CLR: *mut u32 = 0x5000_050C as *mut u32;
/// GPIO CFG
const PIN_CNF_DIR_OUTPUT: u32 = 0x0000_0001;
/// LEDS: PARTICLE_BORON
const RED_LED: u32 = 13;
#[allow(dead_code)]
const BLUE_LED: u32 = 14;
#[allow(dead_code)]
const GREEN_LED: u32 = 15;

fn run_nops(x: u32) {
    for _ in 1..x {
        unsafe {
            asm!("nop");
        }
    }
}

fn main() -> ! {
    let cfg: u32;

    unsafe {
        core::ptr::write_volatile(
            REG_P0_PIN_CNF_BASE.offset(RED_LED as isize),
            PIN_CNF_DIR_OUTPUT,
        );
    }

    cfg = 1 << RED_LED;

    loop {
        unsafe {
            // Should blink red...
            core::ptr::write_volatile(REG_P0_OUT_SET, cfg);
        }
        run_nops(100000);

        unsafe {
            core::ptr::write_volatile(REG_P0_OUT_CLR, cfg);
        }
        run_nops(100000);
    }
}
