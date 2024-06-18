#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux6::{entry, rprint, rprintln};

#[entry]
fn main() -> ! {
    aux6::init();

    rprintln!("Hello, world!");

    loop {}
}
