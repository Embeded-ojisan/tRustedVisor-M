//! Secure World から Non‑Secure へ公開する簡易ハイパコール実装。

/// Veneer (C) 側から参照されるシンボル。
/// 今は常に "VM0" として 0 を返すだけ。
#[no_mangle]
pub extern "C" fn hv_current_vm() -> u32 {
    0
}