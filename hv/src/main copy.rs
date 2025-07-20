#![no_std]
#![no_main]
#![feature(naked_functions)]

use cortex_m_rt::entry;

mod hv;
mod hypercall;
mod panic;

#[entry]
fn main() -> ! {
    hv::init_sau_mpu();
    unsafe { hv::init_vm_table() };
    hv::start_systick(64_000);
    hv::start_first_vm();

    // diverging tail: stay in WFI
    loop { cortex_m::asm::wfi(); }

    // 3️⃣ SysTick (1ms) を起動 — PendSV ハンドルへ繋がる
    hv::start_systick(64_000); // 64 MHz クロック想定で 64_000 = 1 ms

    // 4️⃣ 最初の VM へジャンプ（まだダミー）
    hv::start_first_vm();
}