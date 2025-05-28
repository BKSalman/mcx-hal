#![no_std]
#![no_main]

use eio06::Write;
use mcx_hal::prelude::*;

use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let port1 = Port1::new(unsafe { pac::port::PORT1::instance() });

    let mut lpuart4 = LpUart::new(
        unsafe { pac::lpuart::LPUART4::instance() },
        LpUartPins {
            tx: port1.p9,
            rx: port1.p8,
        },
    );

    lpuart4.configure(|i| {
        i.set_baud(&BaudRate::new(FIRC::default().freq(), 115200).unwrap());
        i.set_rx_fifo(Some(0));
    });
    lpuart4.set_enable(LpUartDirection::TX, true);
    lpuart4.set_enable(LpUartDirection::RX, true);

    loop {
        writeln!(lpuart4, "Hello World!").unwrap();
    }
}
