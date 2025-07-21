#![no_std]
#![no_main]

#![feature(abi_cmse_nonsecure_call)]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

mod hv;
mod hypercall;
mod panic;

#[entry]
fn main() -> ! {
    hprintln!("Hello from hv!");

    unsafe {
        hv::init_sau_mpu();
        hv::init_vm_table();
    }

    hprintln!("Hello from hv2!");
    hv::start_systick(64_000);   // 1 ms tick
    hv::start_first_vm();        // ← ここから先は戻らない (!)

    // ↓↓↓ 以下は実行されないので削除するか #[allow(unreachable_code)]
    // loop { cortex_m::asm::wfi(); }
}