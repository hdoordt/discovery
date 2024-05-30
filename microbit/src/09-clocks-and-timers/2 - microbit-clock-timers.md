# Micro:bit v2 Clock and Timer Peripherals


The micro:bit v2 is powered by the nRF52833 microcontroller, which integrates several peripherals for managing time, clocks, and timers. 

## Real-Time Counter (RTC)

The nRF52833 includes three Real-Time Counters: RTC0, RTC1, and RTC2. These are low-power counters designed for timekeeping in various applications.

### Features
- Low Power Consumption: Ideal for battery-powered applications.
- Event and Interrupt Generation: Capable of generating events and interrupts at specified intervals.
- 24-bit Counter:*Provides a broad range for time intervals.

### Functions
- Timekeeping: Maintains the current time and date.
- Periodic Interrupts: Generates interrupts at regular intervals for time-based operations.
- Alarms: Sets alarms to trigger specific actions at predefined times.


## Timers

The nRF52833 features five general-purpose timers: TIMER0, TIMER1, TIMER2, TIMER3, and TIMER4. These timers are versatile and can be used for a range of timing applications.

### Features
- 32-bit Resolution: High-resolution timing capabilities.
- Multiple Modes: Operates in one-shot or continuous modes.
- Capture/Compare Registers: Allows for precise timing measurements and event triggering.

### Functions
- Delays: Creates precise delays in code execution.
- Event Scheduling: Schedules events to occur at specific intervals.
- Pulse Width Modulation (PWM): Generates PWM signals for controlling peripherals like LEDs and motors.


## Pulse Width Modulation (PWM) Module

The nRF52833 microcontroller includes a PWM module, essential for generating PWM signals used in controlling devices such as servos and LEDs.

### Features
- Multiple Channels: Can drive multiple output channels.
- Configurable Duty Cycle: Adjusts the power delivered to the peripheral.
- High Frequency: Suitable for generating high-frequency PWM signals.

### Functions
- LED Dimming: Controls the brightness of LEDs.
- Motor Control: Adjusts the position or speed of motors.
- Audio Output: Generates audio signals through PWM modulation.

Next we will go through an example of manipulating raw timer registers to control a delay.
