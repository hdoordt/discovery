# Hello, world!

Just a little more of helpful magic before we start doing low level stuff.

Blinking an LED is like the "Hello, world" of the embedded world.

But in this section, we'll run a proper "Hello, world" program that prints stuff to your computer
console.

Go to the `06-hello-world` directory. There's some starter code in it:

``` rust
{{#include src/main.rs}}
```

The `rprintln` macro will format messages and output them over [RTT]. RTT stands
for Real Time Transfer and it's a communication protocol on top of SWD (Serial Wire
Debug) which can be used to send messages from the microcontroller to the debugging host. This
communication is *two way*: the debugging host can also send data back to the microcontroller.

[RTT]: https://wiki.segger.com/RTT

cargo-embed, which is managing the debug session, can receive data sent through this RTT *channel* and
print it to the screen.

The RTT protocol works by reserving a 'Control Block' in the device's RAM, which describes a number of [circular buffers] that also live in device RAM, either for sending or for receiving data.
The control block always starts with an ID, which allows for the Control Block to be detected by the debugger.

[circular buffers]: https://en.wikipedia.org/wiki/Circular_buffer

For the device, sending data over RTT is as simple as copying the bytes into the correct buffer and updating that ring buffer's write pointer.
The debugger can then detect the update, read the data, and update the read pointer of the ring buffer.
Cargo-embed can be configured to display RTT data in its console using the `Embed.toml` configuration file:

```toml
{{#include Embed.toml:9:10}}
```

We will now build and run the application, `cargo embed`. It will show the RTT output in the terminal:

```
 Terminal                                                                                                                       
09:36:54.003: Hello, world!
```

Awesome, right? Feel free to use `rprintln` as a logging tool in the coming sections.

Next: That's not all! The `rprint!` macros are not the only thing that uses the RTT. `:-)`
