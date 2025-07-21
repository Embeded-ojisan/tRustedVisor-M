//! Hypervisor core: SAU/MPU, VMContext, SysTick & PendSV switcher
//! **注意**: ここは `#![no_std]` 環境。標準ライブラリ (`std`) は使えません。

use cortex_m::peripheral::{SAU, SYST};   // SCB を使わないなら外す

use cortex_m_semihosting::hprintln;

const MAX_VMS: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VmContext {
    r4_r11: [u32; 8],
    psp_ns: u32,
    control_ns: u32,
}

const EMPTY_VM: VmContext = VmContext { r4_r11: [0; 8], psp_ns: 0, control_ns: 0 };

pub struct VmTable { vms: [VmContext; MAX_VMS], current: usize }

#[no_mangle]
static mut VM_TABLE: VmTable = VmTable { vms: [EMPTY_VM; MAX_VMS], current: 0 };

//────────────────── SAU / MPU (stub) ──────────────────
/// Secure Attribution Unit 初期化（Flash + SRAM を Non-Secure に）
pub unsafe fn init_sau_mpu() {
    use cortex_m::peripheral::sau::{Rnr, Rbar, Rlar, Ctrl};

/* 0x0020_0000 – 0x0027_FFFF : Non-Secure Flash 512 KiB */
const NS_FLASH_BASE  : u32 = 0x0020_0000;
const NS_FLASH_LIMIT : u32 = 0x0027_FFFF;

/* 0x2000_0000 – 0x2002_FFFF : Non-Secure SRAM 192 KiB (ゆとり) */
const NS_SRAM_BASE   : u32 = 0x2000_0000;
const NS_SRAM_LIMIT  : u32 = 0x2002_FFFF;

let sau = &*cortex_m::peripheral::SAU::PTR;
sau.rnr .write(Rnr (0));         // Flash
sau.rbar.write(Rbar(NS_FLASH_BASE));
sau.rlar.write(Rlar(NS_FLASH_LIMIT | 1));

sau.rnr .write(Rnr (1));         // SRAM
sau.rbar.write(Rbar(NS_SRAM_BASE));
sau.rlar.write(Rlar(NS_SRAM_LIMIT | 1));

sau.ctrl.write(Ctrl(1));
core::arch::asm!("dsb sy; isb sy");   // ← 忘れずに

    /* SAU ON */
    sau.ctrl.write(Ctrl(1));
}

//────────────────── VM Table 初期化 ──────────────────
pub unsafe fn init_vm_table() {
    for (i, vm) in VM_TABLE.vms.iter_mut().enumerate() {
        let stack_top = 0x2000_8000 + (i as u32) * 0x1000;
        vm.psp_ns = stack_top - 0x20; // 8 words frame
        vm.control_ns = 0x2;          // NS + PSP
        vm.r4_r11 = [0; 8];
    }
}

//────────────────── SysTick ──────────────────
pub fn start_systick(ticks: u32) {
    assert!(ticks > 0 && ticks < 0x0100_0000);
    let syst = unsafe { &*SYST::PTR };
    unsafe {
        syst.rvr.write(ticks);
        syst.cvr.write(0);
        syst.csr.write((1<<0)|(1<<1)|(1<<2));
    }
}

#[export_name = "SysTick"]
pub unsafe extern "C" fn systick_handler() {
    const ICSR_PENDSVSET: u32 = 1 << 28;
    hprintln!("Hello from SysTick!");
//    (*SCB::PTR).icsr.write(ICSR_PENDSVSET);
}

//────────────────── PendSV global ASM ──────────────────
core::arch::global_asm!(r#"
    .thumb
    .syntax unified
    .global PendSV
PendSV:
    mrs r0, psp_ns
    ldr r1, =VM_TABLE
    ldr r2, [r1, #8]
    lsls r2, r2, #6   @ *64
    adds r1, r1, r2
    stmia r1!, {{r4-r11}}
    ldr r1, =VM_TABLE
    ldr r2, [r1, #8]
    adds r2, #1
    ands r2, #1
    str  r2, [r1, #8]
    bx   lr
"#);

//────────────────── ブート時 ──────────────────
#[inline(never)]
pub fn start_first_vm() -> ! {
    const VM0_VTOR: u32 = 0x0020_0000;
    let msp_ns   = unsafe { *(VM0_VTOR as *const u32) };
    let reset_ns = unsafe { *((VM0_VTOR + 4) as *const u32) } | 1; // Thumb

    /* VTOR_NS と MSP_NS を設定 */
    unsafe {
        core::ptr::write_volatile(0xE002_ED08 as *mut u32, VM0_VTOR);
        core::arch::asm!("dsb sy; isb sy");
        core::arch::asm!("msr MSP_NS, {0}", in(reg) msp_ns);
        core::arch::asm!(
            "bxns {entry}",
            entry = in(reg) reset_ns,
            options(noreturn)
        );
    }
}