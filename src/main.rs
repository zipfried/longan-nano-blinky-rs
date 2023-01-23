#![no_std]
#![no_main]

use core::panic::PanicInfo;
use gd32vf103xx_hal::{delay::McycleDelay, pac::Peripherals, prelude::*};
use longan_nano::led::{rgb, Led};
use riscv_rt::entry;

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut delay = McycleDelay::new(&rcu.clocks);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);
    let (mut r, mut g, mut b) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);

    g.off();
    b.off();

    loop {
        r.on();
        delay.delay_ms(500);

        r.off();
        delay.delay_ms(500);
    }
}
