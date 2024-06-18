# Exercise: Implementing a Raw Register Delay on the micro:bit v2

### Objective
The goal of this exercise is to modify the provided example program to implement a delay function using raw register manipulation of the TIMER0 peripheral in the micro:bit v2. This will help you understand how to directly interact with hardware registers for precise timing control.

### Led-roulette revisisted

Here we are revisiting the led-roulette example from a previous chapter.

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

const PIXELS: [(usize, usize); 16] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0,0);

    loop {
        for current_led in PIXELS.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, leds, 30);
            last_led = *current_led;
        }
    }
}
```

### Steps to Implement Raw Register Delay

1. Initialization of Peripherals:
   - Ensure that the TIMER0 peripheral is correctly initialized for timer operations. This includes setting the timer mode, bit mode, and prescaler.

2. Define the Delay Function:
   - Implement a custom delay function that uses raw register manipulation to create a delay. This function will directly write to and read from the TIMER0 registers to manage the timing.

3. Clear and Start the Timer:
   - Within the delay function, clear the timer to reset the count and start the timer to begin counting.

4. Set the Compare Register:
   - Configure the compare register to set the delay duration. The timer will count until it reaches this value.

5. Wait for the Compare Event:
   - Implement a loop that waits until the timer reaches the value set in the compare register, indicating that the delay period has elapsed.

6. Clear the Compare Event and Stop the Timer:
   - Once the delay period is over, clear the compare event flag and stop the timer to reset its state.

### Steps to Implement the Timer Configuration

1. Access the TIMER0 Peripheral:
   - Retrieve a reference to the TIMER0 peripheral from the Peripheral Access Crate (PAC). This step is crucial as it provides access to the hardware registers needed for configuration.

2. Set the Timer Mode:
   - Configure the TIMER0 to operate in timer mode. This involves writing to the mode register to ensure the peripheral functions as a timer, which counts up at a regular interval.

3. Set the Bit Mode:
   - Configure the TIMER0 to use a 32-bit counter. This setting allows the timer to count higher values before overflowing, which is important for creating longer delays.

4. Set the Prescaler:
   - Configure the prescaler to divide the input clock frequency appropriately. This involves setting the prescaler register to achieve the desired timer clock rate, ensuring that each tick of the timer represents a specific time unit (e.g., 1 microsecond).

### Hint

Use these Rust crates to build the solution.

```rust
use cortex_m_rt::entry;
use nrf52833_hal::pac::{self, TIMER0};
use nrf52833_hal::gpio::Level;
use embedded_hal::digital::OutputPin;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
```

### Conclusion

By following these steps, you will implement a custom delay function and timer using raw register manipulation of the TIMER0 peripheral. This exercise will help you understand low-level hardware interactions and precise timing control on the micro:bit v2.