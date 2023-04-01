//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;
static mut RAM: u32 = 1;
#[link_section = "sram4"]
static mut SRAM4: u32 = 2;

#[entry]
fn main() -> ! {
    info!("Program start");

    info!("ram: {}", unsafe { RAM });
    info!("sram4: {}", unsafe { SRAM4 });

    // Need some actual write access to this variable, otherwise rustc optimises it like a
    // const and never accesses RAM.
    unsafe { RAM += 1 };
    unsafe { SRAM4 += 1 };
    loop {}
}

// End of file
