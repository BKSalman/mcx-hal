#![no_std]
#![no_main]

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use cortex_m::interrupt::Mutex;
use mcx_hal::pac::interrupt;
use mcx_hal::prelude::*;

use panic_halt as _;

type Btn = Input<PortPin<1, 3>>;

static FLAG_BTN_PRESSED: AtomicBool = AtomicBool::new(false);
static BTN: Mutex<RefCell<Option<Btn>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let port1 = Port1::new(unsafe { pac::port::PORT1::instance() });
    let port0 = Port0::new(unsafe { pac::port::PORT0::instance() });
    let mut gpio1 = GPIO::new(unsafe { pac::gpio::GPIO1::instance() });
    let mut gpio0 = GPIO::new(unsafe { pac::gpio::GPIO0::instance() });

    let led = gpio0.output(port0.p10);
    let mut btn = gpio1.input(port1.p3);

    btn.mut_pin().floating();
    btn.mut_pin().analog(false);
    btn.set_interrupt_config(GPIOIRQConfig::InterruptFallingEdge);
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::GPIO10) }
    unsafe { cortex_m::interrupt::enable() }

    cortex_m::interrupt::free(|cs| BTN.borrow(cs).borrow_mut().replace(btn));

    led.set();

    loop {
        if FLAG_BTN_PRESSED.swap(false, Ordering::Relaxed) {
            led.clear();
        } else {
            led.set();
        }
    }
}

#[interrupt]
unsafe fn GPIO10() {
    cortex_m::interrupt::free(|cs| {
        BTN.borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt_flag()
    });

    FLAG_BTN_PRESSED.store(true, Ordering::Relaxed);
}
