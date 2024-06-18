# Creating a Delay Using Raw Registers of the Timer on the micro:bit v2

We can manipulate raw registers of the TIMER peripheral in the micro:bit v2 for more granular control. The code provided demonstrates the use of these registers to implement a delay function in Rust.  

### Consider the following code

```rust
fn delay(timer: &TIMER0, ms: u32) {
    unsafe {
        timer.tasks_clear.write(|w| w.bits(1));        // Clear the timer
        timer.tasks_start.write(|w| w.bits(1));        // Start the timer
        timer.cc[0].write(|w| w.bits(ms * 1_000));     // Set the compare register (1ms tick)

        while timer.events_compare[0].read().bits() == 0 {}  // Wait for the compare event

        timer.events_compare[0].write(|w| w.bits(0));   // Clear the compare event
        timer.tasks_stop.write(|w| w.bits(1));          // Stop the timer
    }
}
```

### Function Definition
```rust
fn delay(timer: &TIMER0, ms: u32) {
```
- `fn delay`: Defines a new function named `delay`.
- `timer: &TIMER0`: The function takes a reference to a `TIMER0` peripheral as an argument.
- `ms: u32`: The function also takes an unsigned 32-bit integer representing the delay in milliseconds.

### Entering Unsafe Block
```rust
    unsafe {
```
- `unsafe`: Enters an unsafe block, allowing for direct manipulation of hardware registers.

### Clearing the Timer
```rust
        timer.tasks_clear.write(|w| w.bits(1));
```
- `timer.tasks_clear.write(|w| w.bits(1))`: Writes a `1` to the `tasks_clear` register, which resets the timer's count to zero.

### Starting the Timer
```rust
        timer.tasks_start.write(|w| w.bits(1));
```
- `timer.tasks_start.write(|w| w.bits(1))`: Writes a `1` to the `tasks_start` register, which starts the timer.

### Setting the Compare Register
```rust
        timer.cc[0].write(|w| w.bits(ms * 1_000));
```
- `timer.cc[0].write(|w| w.bits(ms * 1_000))`: Writes the value `ms * 1_000` to the `cc[0]` (compare capture) register. Since the timer operates at a frequency of 1 MHz, each tick corresponds to 1 microsecond. Therefore, to create a delay of `ms` milliseconds, the compare register is set to `ms * 1_000` microseconds.

### Waiting for the Compare Event
```rust
        while timer.events_compare[0].read().bits() == 0 {}
```
- `while timer.events_compare[0].read().bits() == 0 {}`: Enters a loop that continuously checks the `events_compare[0]` register. This loop keeps running until the compare event occurs, which means the timer count has reached the value set in the `cc[0]` register.

### Clearing the Compare Event
```rust
        timer.events_compare[0].write(|w| w.bits(0));
```
- `timer.events_compare[0].write(|w| w.bits(0))`: Writes a `0` to the `events_compare[0]` register, clearing the compare event flag.

### Stopping the Timer
```rust
        timer.tasks_stop.write(|w| w.bits(1));
```
- `timer.tasks_stop.write(|w| w.bits(1))`: Writes a `1` to the `tasks_stop` register, stopping the timer.

### Exiting the Unsafe Block
```rust
    }
}
```
- The function exits the unsafe block and ends.

### Summary

The `delay` function sets up and uses the TIMER peripheral on the micro:bit v2 to create a precise delay in milliseconds. The steps are:
1. Clear the timer to reset the count.
2. Start the timer to begin counting.
3. Set the compare register to the desired delay in microseconds.
4. Wait for the compare event indicating the timer has reached the desired count.
5. Clear the compare event to reset the event flag.
6. Stop the timer to halt counting.

Next we will configure our timer to be used in an application you'll build as an exercise.
