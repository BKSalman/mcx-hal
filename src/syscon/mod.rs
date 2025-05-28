//! System Controller.
//!
//! This module provides the peripheral control for NXP MCX MCUs.

use crate::private::Sealed;

/// Peripheral reset trait.
pub trait PeripheralRST: Sealed {
    /// Assert or release the reset signal.
    fn assert_reset(&mut self, release: bool);

    /// Reset the peripheral.
    fn reset(&mut self) {
        self.assert_reset(true);
        self.assert_reset(false);
    }
}

/// Peripheral clock control trait.
pub trait PeripheralCC: Sealed {
    /// Enable or disable the peripheral clock.
    fn enable_clock(&mut self, enable: bool);
}

/// Peripheral enable trait.
pub trait PeripheralEn: Sealed {
    /// Enable or disable the peripheral.
    fn enable(enable: bool);
}

#[cfg(feature = "mcxn")]
macro_rules! periph_syscon {
    ( $( ($(virt: $virt:ident)? $(periph: $periph:ty)?, $n:expr, $bit:expr $(,hRST: $hRST:expr)? $(,hCC: $hCC:expr)? $(,hACC: $hACC:expr)?) )+ ) => {};
    ( $( ($(virt: $virt:ident)? $(periph: $periph:ty)?, instance: $instance:expr, bit: $bit:expr $(,hRST: $hRST:expr)? $(,hCC: $hCC:expr)?) )+ ) => {
        periph_syscon!(@virtual_peripherals $($($virt)?)+);
        $($(
            impl crate::private::Sealed for $periph {}
        )?)+

        pub use virtual_peripherals::*;
        $(
            periph_syscon!(@impl_rst periph_syscon!(@name $(virt: $virt)? $(periph: $periph)?), $instance, $bit $(,hRST: $hRST)?);
            periph_syscon!(@impl_cc  periph_syscon!(@name $(virt: $virt)? $(periph: $periph)?), $instance, $bit $(,hCC: $hCC)?);
        )+
    };
    (@name $(virt: $virt:ident)? $(periph: $periph:ty)?) => {
        $($virt)?
        $($periph)?
    };
    (@virtual_peripherals $($virt:ident)*) => {
        pub mod virtual_peripherals {
            $(
                pub struct $virt;
                impl crate::private::Sealed for $virt {}
            )*
        }
    };
    (@impl_rst $name:ty, $instance:expr, $bit:expr) => {};
    (@impl_rst $name:ty, $instance:expr, $bit:expr, hRST: $hRST:expr) => {
        impl crate::syscon::PeripheralRST for $name {
            #[inline(always)]
            fn assert_reset(&mut self, release: bool) {
                let reg = unsafe {
                    let ptr = crate::pac::syscon::ADDRESSES[0] as *mut u8;
                    // offset to the set/clear register
                    let offset = if release { 0x20usize } else { 0x40usize };
                    crate::pac::common::Reg::<u32, crate::pac::common::W>::from_ptr(ptr.add(0x100usize + $instance * 0x4usize + offset) as _)
                };
                reg.write(|r| *r = 1 << $bit);
            }
        }
    };
    (@impl_cc $name:ty, $instance:expr, $bit:expr) => {};
    (@impl_cc $name:ty, $instance:expr, $bit:expr, hCC: $hCC:expr) => {
        impl crate::syscon::PeripheralCC for $name {
            #[inline(always)]
            fn enable_clock(&mut self, enable: bool) {
                let reg = unsafe {
                    let ptr = crate::pac::syscon::ADDRESSES[0] as *mut u8;
                    // offset to the set/clear register
                    let offset = if enable { 0x20usize } else { 0x40usize };
                    crate::pac::common::Reg::<u32, crate::pac::common::W>::from_ptr(ptr.add(0x0200usize + $instance * 0x4usize + offset) as _)
                };
                reg.write(|r| *r = 1 << $bit);
            }
        }
    };
}

#[cfg(feature = "mcxn")]
pub(crate) use periph_syscon;

#[cfg_attr(feature = "mcxa0", path = "device/a0.rs")]
#[cfg_attr(feature = "mcxa1", path = "device/a1.rs")]
#[cfg_attr(feature = "mcxa2", path = "device/a2.rs")]
#[cfg_attr(feature = "mcxn0", path = "device/n0.rs")]
mod device;
pub use device::*;

#[cfg(any(feature = "mcxa0", feature = "mcxa1", feature = "mcxa2"))]
mod mrcc;
#[cfg(any(feature = "mcxa0", feature = "mcxa1", feature = "mcxa2"))]
use mrcc::periph_mrcc;
