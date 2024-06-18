# `panic!`

The `panic!` macro also sends its output to the RTT!

Change the `main` function to look like this:

``` rust
#[entry]
fn main() -> ! {
    aux6::init();
    panic!("Hello, world!");
}
```

OK, now use `cargo embed` and have a look at the console:

```
 Terminal                                                                                                                       
09:54:30.139: panicked at src/06-hello-world/src/bin/panic.rs:14:5:
09:54:30.139: Hello, world!
```

Cool! Now let's dive a little deeper using GDB:

``` console
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `rust-gdb -x embed.gdb /home/hd/dev/rs/discovery/microbit/target/thumbv7em-none-eabihf/debug/hello-world`
GNU gdb (Fedora Linux) 14.2-2.fc40
Copyright (C) 2023 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-redhat-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/hd/dev/rs/discovery/microbit/target/thumbv7em-none-eabihf/debug/hello-world...
warning: Remote gdbserver does not support determining executable automatically.
RHEL <=6.8 and <=7.2 versions of gdbserver do not support such automatic executable detection.
The following versions of gdbserver support it:
- Upstream version of gdbserver (unsupported) 7.10 or later
- Red Hat Developer Toolset (DTS) version of gdbserver from DTS 4.0 or later (only on x86_64)
- RHEL-7.3 versions of gdbserver (on any architecture)
0x00000498 in panic_rtt_target::panic (info=0x2001ffac) at src/lib.rs:68
68	        compiler_fence(SeqCst);
Breakpoint 1 at 0x15c: file src/06-hello-world/src/main.rs, line 10.
Note: automatically using hardware breakpoints for read-only addresses.
Breakpoint 2 at 0x2f0: file src/lib.rs, line 1053.
Breakpoint 3 at 0x2b1a: file src/lib.rs, line 1046.
Resetting and halting target
Target halted
(gdb) 


```

All right. The firmware is all loaded up, and the device is halted right after having been reset.
Let's start execution until the first breakpoint,.
We'll use short command names to save typing, enter `c` then the `Enter` or `Return` key:
```
(gdb) c
Continuing.
```

If all is well you'll see some new output in cargo-embed's terminal.

``` console
 Terminal                                                                                                                       
10:26:29.665: panicked at src/06-hello-world/src/main.rs:14:5:
10:26:29.665: Hello, world!
10:27:09.653: panicked at src/06-hello-world/src/main.rs:14:5:
10:27:09.653: Hello, world!
```

Note the timestamps. The first panic output is from before GDB reset and halted the device.

Then type `Ctrl-C` which breaks out of a loop in the runtime:
``` text
^C
Program received signal SIGINT, Interrupt.
core::sync::atomic::compiler_fence (order=core::sync::atomic::Ordering::SeqCst)
    at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3736
3736	            SeqCst => intrinsics::atomic_singlethreadfence_seqcst(),
(gdb) 
```

Ultimately, `panic!` is just another function call so you can see it leaves behind
a trace of function calls. This allows you to use `backtrace` or just `bt` and to see
call stack that caused the panic:

``` text
(gdb) bt
#0  core::sync::atomic::compiler_fence (order=core::sync::atomic::Ordering::SeqCst)
    at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/sync/atomic.rs:3740
#1  0x00000498 in panic_rtt_target::panic (info=0x2001ffac) at src/lib.rs:68
#2  0x0000196e in core::fmt::num::exp_u128 () at library/core/src/panicking.rs:72
#3  0x000019d6 in core::fmt::num::exp_u128 () at library/core/src/panicking.rs:145
#4  0x0000017e in panic::__cortex_m_rt_main () at src/06-hello-world/src/bin/panic.rs:14
#5  0x00000160 in panic::__cortex_m_rt_main_trampoline () at src/06-hello-world/src/bin/panic.rs:10
```

We can run `up` in GDB to navigate one frame up in the call stack, and then print the code surrounding the call to `core::sync::atomic::compiler_fence` using `list`:

```
(gdb) up
#1  0x00000498 in panic_rtt_target::panic (info=0x2001ffac) at src/lib.rs:68
68	        compiler_fence(SeqCst);
(gdb) list
53	#[cfg(feature = "cortex-m")]
54	#[inline(never)]
55	#[panic_handler]
56	fn panic(info: &PanicInfo) -> ! {
57	    use cortex_m::interrupt;
58	
59	    interrupt::disable();
60	
61	    if let Some(mut channel) = unsafe { UpChannel::conjure(0) } {
62	        channel.set_mode(ChannelMode::BlockIfFull);
63	
64	        writeln!(channel, "{}", info).ok();
65	    }
66	
67	    loop {
68	        compiler_fence(SeqCst);
69	    }
70	}
71	
72	#[cfg(not(any(feature = "cortex-m")))]
73	compile_error!(concat!(
74	    "You must specify a platform feature for panic-rtt-target, such as 'cortex-m'.\r\n",
75	    "Example:\r\n",
76	    "  # Cargo.toml\r\n",
77	    "  panic-rtt-target = { version = \"x.y.z\", features = [\"cortex-m\"] }\r\n"
78	));
```

Let's see, when we ran `bt` before, it showed that the second entry of the call stack (`#1`), is at line 68 of `src/lib.rs` in `panic_rtt_target::panic`. And here we are: a function called `panic`, that ends in an endless loop which repeaedly   calls a function called `compiler_fence`. It's annotated by the `#[panic_hander]` attribute, which makes the Rust compiler see that it needs to call this function on `panic!`. You can write your own panic handler, too! Just make sure that there's no other function annotated with `#[panic_handler]` in your crate or your dependencies, and ensure that it has the correct signature.

Another thing we can do is catch the panic *before* it does the logging.
So we'll do several things; reset to the beginning, disable breakpoint 1,
set a new breakpoint at `rust_begin_unwind`, list the break points and then continue:

``` text
(gdb) monitor reset halt
Resetting and halting target
Target halted

(gdb) disable 1

(gdb) break rust_begin_unwind
Breakpoint 4 at 0x41e: file src/lib.rs, line 59.

(gdb) info break
Num     Type           Disp Enb Address    What
1       breakpoint     keep n   0x0000015c in panic::__cortex_m_rt_main_trampoline at src/06-hello-world/src/bin/panic.rs:10
	breakpoint already hit 1 time
2       breakpoint     keep y   0x000002f0 in cortex_m_rt::DefaultHandler_ at src/lib.rs:1053
3       breakpoint     keep y   0x00002b1a in cortex_m_rt::HardFault_ at src/lib.rs:1046
4       breakpoint     keep y   0x0000041e in panic_rtt_target::panic at src/lib.rs:59

(gdb) c
Continuing.

Breakpoint 4, panic_rtt_target::panic (info=0x2001ffac) at src/lib.rs:59
59	    interrupt::disable();
```

You'll notice that nothing got printed on cargo-embed's console this time. If
you resume the program using `continue` then a new line will be printed.

In a later section we'll look into other communication protocols.

Finally, enter the `q` command to quit, then press `y` to confirm.

``` text
(gdb) q
A debugging session is active.

	Inferior 1 [process 1] will be detached.

Quit anyway? (y or n) y
Detaching from program: /home/hd/dev/rs/discovery/microbit/target/thumbv7em-none-eabihf/debug/panic, process 1
Ending remote debugging.
[Inferior 1 (process 1) detached]
```

All done!
