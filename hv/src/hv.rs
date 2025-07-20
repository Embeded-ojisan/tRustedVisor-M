//! Hypervisor core: SAU/MPU, VMContext, SysTick & PendSV switcher
//! **注意**: ここは `#![no_std]` 環境。標準ライブラリ (`std`) は使えません。

use cortex_m::peripheral::{SCB, SYST};

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
pub fn init_sau_mpu() {}

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
    (*SCB::PTR).icsr.write(ICSR_PENDSVSET);
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
pub fn start_first_vm() -> ! { loop { cortex_m::asm::nop(); } }