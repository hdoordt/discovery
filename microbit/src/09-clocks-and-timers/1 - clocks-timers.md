# Clocks and Timers

PCs use an RTC (Real-Time Clock) that is typically battery powered. A PC's BIOS (Basic Input/Output System) uses the RTC to set the system clock at boot. But once the PC is connected to the internet, it's able to connect and synchronize its system clock to Network Time Protocol (NTP) servers.

With embedded systems, many times we will not have the luxury of an internet connection, so we rely on alternative timekeeping methods.

### Real-Time Clock (RTC):
   - Embedded systems often use an RTC, similar to PCs, to keep track of the current time and date. This RTC is typically battery-powered to maintain timekeeping even when the system is powered off.
   - The RTC provides a reliable time source for scheduling tasks, logging events, and maintaining accurate timestamps, crucial for applications like data logging, time-stamped transactions, and alarm systems.

### External Oscillator Crystals:
   - Many embedded systems use external oscillator crystals to generate precise clock signals for the microcontroller. These crystals provide a stable frequency reference that ensures accurate timing for system operations.
   - While they don't keep real-time, these crystals are essential for the precise timing required in tasks such as communication protocols (e.g., UART, SPI, I2C), PWM generation, and sensor data sampling.

### Microcontroller Internal Clocks:
   - Microcontrollers often have internal oscillators and clock sources that can be used when high precision is not as critical. These internal clocks are convenient but typically less accurate than external crystals.
   - Some microcontrollers offer calibration features to adjust the internal oscillator frequency based on temperature or other factors.


Timers are essential components in embedded systems, providing critical functions such as generating delays, scheduling events, and measuring time intervals. Here’s a detailed breakdown of the different types and uses of timers:

### Purpose and Function:
   - Timers in embedded systems are used to manage time-dependent tasks. They can operate in various modes to cater to different requirements, such as one-shot, periodic, and PWM (Pulse Width Modulation) modes.

### Types of Timers:

   - Watchdog Timer:
     - Function: Ensures system reliability by resetting the system if the software fails to reset the timer within a specified period, indicating a potential malfunction.
     - Use Case: Critical in safety and reliability applications where system hang-ups or crashes must be detected and recovered automatically.

   -  General-Purpose Timer:
     -  Function: Versatile timers used for creating precise delays, generating output signals at specific intervals, and timing the duration of events.
     -  Use Case: Utilized in a wide range of applications from simple delay functions to complex timing operations like generating timed interrupts for periodic tasks.

   -  PWM (Pulse Width Modulation) Timer:
     - Function: Generates PWM signals which are used to control the power delivered to devices such as motors, LEDs, and other actuators.
     - Use Case: Essential in applications requiring precise control of power and signal modulation, such as motor speed control, dimming LEDs, and audio signal generation.

### Usage in Applications:

   - Delay Generation:
     - Description: Timers can create precise delays in software, which is crucial for applications requiring accurate timing control. Delays can range from microseconds to milliseconds, depending on the timer resolution and configuration.
     - Example: Implementing debounce logic for mechanical buttons, where a delay is required to filter out noise.

   - Event Scheduling:
     - Description: Timers can trigger interrupts or events at specific intervals, which is useful for tasks that need to occur periodically. This can be configured to execute routines at regular intervals without requiring constant CPU intervention.
     - Example: Periodically sampling sensor data, updating display screens, or performing maintenance tasks in real-time systems.

   - Time Measurement:
     - Description: Timers can measure the duration of events with high precision, providing valuable information for performance profiling, event timing, and other analytical purposes.
     - Example: Measuring the pulse width of an incoming signal in a communication protocol or calculating the time taken by a specific operation to optimize code performance.

Next we will dive into the capabilities of the Microbit v2 clock and timer peripherals. 
