// This example demonstrates the basic functionality of this crate.
// It simply panics and prints the panic message on the UART.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;

use bsp::pins::common::{P0, P1};
imxrt_uart_panic::register!(LPUART6, P1, P0, 115200);

#[bsp::rt::entry]
fn main() -> ! {
    panic!("Foo!");
}
