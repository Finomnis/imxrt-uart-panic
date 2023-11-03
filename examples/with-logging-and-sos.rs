// This example demonstrates the usage of this crate in a more complex scenario.
// Note the additional argument that hands the panic over to `teensy4_panic::sos`
// after printing to UART.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;

use bsp::pins::common::{P0, P1};
imxrt_uart_panic::register!(LPUART6, P1, P0, 115200, teensy4_panic::sos);

#[rtic::app(device = teensy4_bsp)]
mod app {
    use teensy4_bsp as bsp;

    use bsp::board;
    use bsp::hal;
    use bsp::logging;

    use hal::gpt;

    use embedded_hal::serial::Write;

    const LOG_POLL_INTERVAL: u32 = board::PERCLK_FREQUENCY / 100;
    const LOG_DMA_CHANNEL: usize = 0;

    #[local]
    struct Local {
        poll_log: hal::pit::Pit3,
        log_poller: logging::Poller,
        gpt1: hal::gpt::Gpt1,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut dma,
            pit: (_, _, _, mut poll_log),
            pins,
            lpuart6,
            mut gpt1,
            ..
        } = board::t40(cx.device);

        // Logging
        let log_dma = dma[LOG_DMA_CHANNEL].take().unwrap();
        let mut log_uart = board::lpuart(lpuart6, pins.p1, pins.p0, 115200);
        for &ch in "\r\n===== i.MX RT UART Panic example =====\r\n\r\n".as_bytes() {
            nb::block!(log_uart.write(ch)).unwrap();
        }
        nb::block!(log_uart.flush()).unwrap();
        let log_poller =
            logging::log::lpuart(log_uart, log_dma, logging::Interrupts::Enabled).unwrap();
        poll_log.set_interrupt_enable(true);
        poll_log.set_load_timer_value(LOG_POLL_INTERVAL);
        poll_log.enable();

        // Countdown
        gpt1.set_clock_source(gpt::ClockSource::PeripheralClock);
        gpt1.set_divider(1);
        gpt1.set_output_compare_count(gpt::OutputCompareRegister::OCR1, board::PERCLK_FREQUENCY);
        gpt1.set_mode(gpt::Mode::Restart);
        gpt1.set_reset_on_enable(true);
        gpt1.set_output_interrupt_on_compare(gpt::OutputCompareRegister::OCR1, true);
        gpt1.enable();

        (
            Shared {},
            Local {
                log_poller,
                poll_log,
                gpt1,
            },
        )
    }

    #[task(binds = GPT1, local = [gpt1, value: u32 = 11])]
    fn countdown(cx: countdown::Context) {
        let countdown::LocalResources { value, gpt1, .. } = cx.local;

        gpt1.clear_elapsed(gpt::OutputCompareRegister::OCR1);

        *value -= 1;
        if *value == 0 {
            panic!("Countdown over!");
        }

        log::info!("Countdown: {}", value);
    }

    #[task(binds = PIT, priority = 1, local = [poll_log, log_poller])]
    fn logger(cx: logger::Context) {
        let logger::LocalResources {
            poll_log,
            log_poller,
            ..
        } = cx.local;

        if poll_log.is_elapsed() {
            poll_log.clear_elapsed();
            log_poller.poll();
        }
    }
}
