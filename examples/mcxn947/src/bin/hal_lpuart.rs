#![no_std]
#![no_main]

use defmt_rtt as _;
use eio06::Write;
use mcx_hal::prelude::*;

#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("{}", info);
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut scg = SCG::without_pins(unsafe { pac::scg::SCG0::instance() });
    let cfg = SCGConfig {
        firc_fclk_en: true,
        ..Default::default()
    };
    scg.freeze(&cfg).unwrap();

    defmt::info!("alo");

    let port1 = Port1::new(unsafe { pac::port::PORT1::instance() });

    setup_fro_hf_divider(Some(0));
    // setup_lpuart2_clock_source(mcx_hal::syscon::MRCCClockSource::FroHfDiv);
    // setup_lpuart2_divider(Some(0));

    let mut lpuart4 = LpUart::new(
        unsafe { pac::lpuart::LPUART4::instance() },
        LpUartPins {
            tx: port1.p9,
            rx: port1.p8,
        },
    );

    defmt::info!("lmao");
    lpuart4.configure(|i| {
        i.set_baud(&BaudRate::new(FIRC::default().freq(), 115200).unwrap());
        i.set_rx_fifo(Some(0));
    });
    lpuart4.set_enable(LpUartDirection::TX, true);
    lpuart4.set_enable(LpUartDirection::RX, true);
    defmt::info!("configured");

    loop {
        writeln!(
            lpuart4,
            "Hello World!, current FIRC {:?}, {}",
            cfg.firc, cfg.firc_fclk_en
        )
        .unwrap();
    }
}
