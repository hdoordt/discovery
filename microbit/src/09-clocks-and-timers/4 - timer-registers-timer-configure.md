# Timer Configuration Code

Now that you know how to create a delay function based on raw registers, we'll configure the TIMER0 peripheral for use in creating delays on the micro:bit v2. This explanation is crucial for understanding how to set up a timer to measure precise time intervals, which is essential for creating accurate delays.

```rust
let timer = &pac.TIMER0;
```
- This line retrieves a reference to the TIMER0 peripheral from the Peripheral Access Crate (PAC). The PAC provides access to the microcontroller's hardware peripherals.

### Configuring TIMER0

```rust
// Configure TIMER0
timer.mode.write(|w| w.mode().timer());          // Set timer mode
timer.bitmode.write(|w| w.bitmode()._32bit());   // Set 32-bit mode
timer.prescaler.write(|w| unsafe { w.bits(4) }); // Set prescaler to 16 (2^4 = 16, for 1 MHz timer clock)
```

#### Setting Timer Mode

```rust
timer.mode.write(|w| w.mode().timer());
```

- This line configures the mode of the TIMER0 peripheral. 
- Accesses the `mode` register of the TIMER0 peripheral and writes to it.
- Sets the mode of the timer to `timer` mode. The TIMER0 can operate in different modes (e.g., counter mode), but here it is set to function as a timer, which counts up at a constant rate.

#### Setting 32-bit Mode

```rust
timer.bitmode.write(|w| w.bitmode()._32bit());
```
- This line sets the bit mode of the TIMER0 peripheral.
- Accesses the `bitmode` register of the TIMER0 peripheral and writes to it.
- Configures the timer to use a 32-bit counter. This allows the timer to count higher values before overflowing, providing a longer delay period if needed.

#### Setting the Prescaler

```rust
timer.prescaler.write(|w| unsafe { w.bits(4) });
```
- This line sets the prescaler value of the TIMER0 peripheral.
- Accesses the `prescaler` register of the TIMER0 peripheral and writes to it.
- Uses an unsafe block to write a raw value (4) to the prescaler register. The prescaler divides the input clock frequency to reduce the timer clock rate.
- Prescaler value of 4: This means the timer clock will be divided by \(2^4 = 16\). Given that the micro:bit's default clock is 16 MHz, this results in a timer clock of 1 MHz (16 MHz / 16). This means each tick of the timer represents 1 microsecond.

### Summary

By configuring TIMER0 in this way, we:
1. Set the timer to operate in timer mode, which counts up at a regular interval.
2. Use a 32-bit counter to allow for a wide range of delay values.
3. Set the prescaler to 4, resulting in a timer clock of 1 MHz. This configuration means each tick of the timer corresponds to 1 microsecond, allowing us to measure time intervals with microsecond precision.

Next we will revist the led-roulette example from a previous chapter and implement this delay.