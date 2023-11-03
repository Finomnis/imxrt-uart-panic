# imxrt-uart-panic

[![Crates.io](https://img.shields.io/crates/v/imxrt-uart-panic)](https://crates.io/crates/imxrt-uart-panic)
[![Crates.io](https://img.shields.io/crates/d/imxrt-uart-panic)](https://crates.io/crates/imxrt-uart-panic)
[![License](https://img.shields.io/crates/l/imxrt-uart-panic)](https://github.com/Finomnis/imxrt-uart-panic/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/imxrt-uart-panic/ci.yml?branch=main)](https://github.com/Finomnis/imxrt-uart-panic/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/imxrt-uart-panic)](https://docs.rs/imxrt-uart-panic)

This crate provides a UART based panic handler for i.MX RT.

Upon panic, it resets the given UART peripheral and writes an error message to it.

## Usage Example

*- examples are intended for the [Teensy 4.0](https://www.pjrc.com/store/teensy40.html), [Teensy 4.1](https://www.pjrc.com/store/teensy41.html) or [Teensy MicroMod](https://www.sparkfun.com/products/16402) board -*


```rust
#![no_std]
#![no_main]

use teensy4_bsp as bsp;

use bsp::pins::common::{P0, P1};
imxrt_uart_panic::register!(LPUART6, P1, P0, 115200);

#[bsp::rt::entry]
fn main() -> ! {
    panic!("Foo!");
}
```
```none
panicked at examples\minimal.rs:11:5:
Foo!
```


Additionally, one can provide a custom panic action, such as `teensy4_panic::sos`,
that will be executed after printing to UART:

```rust
#![no_std]
#![no_main]

use teensy4_bsp as bsp;

use bsp::pins::common::{P0, P1};
imxrt_uart_panic::register!(LPUART6, P1, P0, 115200, teensy4_panic::sos);

#[bsp::rt::entry]
fn main() -> ! {
    panic!("Foo!");
}
```

This crate is fully compatible with other previous usages of the given UART peripheral,
although it might abort transmissions that are already in progress.
