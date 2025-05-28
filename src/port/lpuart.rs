//! LPUART pin define

use crate::port::Port;

pub trait UartRxPin: Port {
    type Module;
    const MUX: u8;
}

pub trait UartTxPin: Port {
    type Module;
    const MUX: u8;
}

// TODO: allow same pins to be used with multiple UART modules
macro_rules! lpuart {
    (pin: $pin:ty, module: $module:ident, signal: RXD, mux: $mux:expr) => {
        impl crate::port::lpuart::UartRxPin for $pin {
            type Module = crate::consts::$module;
            const MUX: u8 = $mux;
        }
    };
    (pin: $pin:ty, module: $module:ident, signal: TXD, mux: $mux:expr) => {
        impl crate::port::lpuart::UartTxPin for $pin {
            type Module = crate::consts::$module;
            const MUX: u8 = $mux;
        }
    };
}
pub(crate) use lpuart;
