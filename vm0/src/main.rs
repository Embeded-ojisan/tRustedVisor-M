#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern "C" { fn hv_hypercall(num: u32, a1: u32, a2: u32, a3: u32) -> u32; }

use core::panic::PanicInfo;

use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    hprintln!("Hello from vm0!");

//    let id = unsafe { hv_hypercall(0, 0, 0, 0) };
    loop {
        // QEMU semihosting 端末へ出力
//        qemu_exit::qemu_println!("Hello from VM{`, ID={}", id, id);
        hprintln!("Hello from vm0loop!");
        for _ in 0..8_000_000 { cortex_m::asm::nop() } // dummy workload
    }
}
