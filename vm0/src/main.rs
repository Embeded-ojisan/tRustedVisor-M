#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern "C" { fn hv_hypercall(num: u32, a1: u32, a2: u32, a3: u32) -> u32; }

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
//    let id = unsafe { hv_hypercall(0, 0, 0, 0) };
    loop {
        // QEMU semihosting 端末へ出力
//        qemu_exit::qemu_println!("Hello from VM{`, ID={}", id, id);
        for _ in 0..8_000_000 { cortex_m::asm::nop() } // dummy workload
    }
}
