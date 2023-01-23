blinky written in rust on Longan Nano (GD32VF103CB)

To flash, you need `cargo-binutils` and `dfu-utils`, then execute following commands.

```
$ cargo objcopy --release -- -O binary blinky.bin
$ dfu-util -a 0 --dfuse-address 0x08000000:leave -D blinky.bin
```
