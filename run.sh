#!/usr/bin/env bash
set -eu

# 1) hv / vm0 をビルド
cargo build -p hv  --release
cargo build -p vm0 --release

# 2) 出力パス
SEC=target/thumbv8m.main-none-eabi/release/hv
NSC=target/thumbv8m.main-none-eabi/release/vm0

# 3) QEMU 実行

qemu-system-arm -d cpu,unimp -D log.txt -machine mps2-an505 -cpu cortex-m33 -nographic -semihosting \
  -device loader,file=$SEC                   \
  -device loader,file=$NSC