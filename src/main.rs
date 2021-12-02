//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]
// needed for alloc
//#![feature(default_alloc_error_handler)]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;
use rp2040_hal as hal;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// needed for alloc
//use alloc_cortex_m::CortexMHeap;
//#[global_allocator]
//static ALLOCATOR: CortexMHeap = CortexMHeap::empty();


#[entry]
fn main() -> ! {
    info!("Program start");

    // needed for alloc
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024; // in bytes
    debug!("start = {:#010x}, size = {:#010x}", start, size);

//    unsafe { ALLOCATOR.init(start, size) }

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut sio = Sio::new(pac.SIO);
    use hal::multicore;
    let mut mc = multicore::Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio);
    unsafe {
        static mut STACK: multicore::Stack<1024> = multicore::Stack{
            mem: [0usize; 1024]
        };
        mc.cores()[1].spawn(test, &mut STACK).unwrap();
    }

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    /*

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();
    */

    delay.delay_ms(1000);
    #[allow(clippy::empty_loop)]
    loop {
        info!("A!");
        /*
        //led_pin.set_high().unwrap();
        delay.delay_ms(501);
        info!("B!");
        //led_pin.set_low().unwrap();
        delay.delay_ms(501);
        */
    }
}

fn test() -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    let core = unsafe { pac::CorePeripherals::steal() };

    let external_xtal_freq_hz = 12_000_000u32;
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.gpio25.into_push_pull_output();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
    //pac.PSM.frce_off.modify(|_, w| w.proc0().set_bit());
}
