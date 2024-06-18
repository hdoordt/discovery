//! Initialization code

#![deny(warnings)]
#![no_std]

use panic_rtt_target as _; // panic handler
use nrf52833_hal as _; // Memory layout

pub use cortex_m_rt::entry;
pub use rtt_target::{rprint, rprintln};

#[inline(never)]
pub fn init() {
    rtt_target::rtt_init_print!();
}
