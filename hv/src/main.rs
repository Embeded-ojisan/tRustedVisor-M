#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello from hv!");
    loop {}
}