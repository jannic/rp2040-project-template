//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use core::arch::global_asm;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;
static mut RAM: u32 = 1;
#[link_section = ".sram4"]
static mut SRAM4: u32 = 4;
#[link_section = ".sram5"]
static mut SRAM5: u32 = 5;

global_asm! {
    "
    .section .text
    .align 2
    data_cpy_table:
     .word __ram4_source__
     .word __ram4_start__
     .word __ram4_end__
     .word __ram5_source__
     .word __ram5_start__
     .word __ram5_end__
     .word 0
    .global __pre_init
    .type __pre_init,%function
    .thumb_func
    __pre_init:
     push {{r4, lr}}
     ldr r4, =data_cpy_table

    1:
     ldmia r4!, {{r1-r3}}
     cmp r1, #0
     beq 2f
     bl data_cpy
     b 1b
    2:
     pop {{r4, pc}}
     data_cpy_loop:
     ldm r1!, {{r0}}
     stm r2!, {{r0}}
     data_cpy:
     cmp r2, r3
     blo data_cpy_loop
     bx lr
    "
}

#[entry]
fn main() -> ! {
    info!("Program start");

    info!("ram: {}", unsafe { RAM });
    info!("sram4: {}", unsafe { SRAM4 });
    info!("sram5: {}", unsafe { SRAM5 });

    // Need some actual write access to this variable, otherwise rustc optimises it like a
    // const and never accesses RAM.
    unsafe { RAM += 1 };
    unsafe { SRAM4 += 1 };
    unsafe { SRAM5 += 1 };
    loop {}
}

// End of file
