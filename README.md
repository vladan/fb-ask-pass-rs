[![Build Status](https://travis-ci.org/gdamjan/fb-ask-pass-rs.svg?branch=master)](https://travis-ci.org/gdamjan/fb-ask-pass-rs)


Primary usage: run in the initramfs (on my archlinux) to ask for the LUKS passphrase, while showing the firmware picture.

# FAQ

## Why Rust?

- It's a learning experience
- Rust compiles to a single binary, depends only on glibc


## How does it work?

- the program reads the firmware image provided by ACPI 5.0 from `/sys/firmware/acpi/bgrt/*` and displays it on the
  framebuffer at the same position (`xoffset`, `yoffset`).
  - afaik, UEFI needs to be enabled. probably quick boot, and full resolution booting too.
- then waits for the user to enter its password, and writes it to a file (for ex. it can write to
  `/crypto_keyfile.bin` which on archlinux is used by default by the `encrypt` initcpio hook). for debug purposes, and when used without any argument, the password is printed on stdout.
- enabling the `fastboot` option on `i915` is recommended.
- look in `arch/` to see how to integrate with archlinux's initcpio system.

## Why not plymouth

- Yes, [Plymouth](https://www.freedesktop.org/wiki/Software/Plymouth/) is probably the better solution. It has much
  more features and is much better tested.
- For example, plymouth stays in control for the whole boot process, and gives out control only before X11/lgoin manager takes over. This program instead, leaves the console in static graphic mode with the image, making it fairly useless.
- This is just a very simple experiment.


# TODO

- some kind of password prompt, and some feedback
- see how to integrate with Dracut and/or other distros
- probably by using the [keyring interface](https://gitlab.com/cryptsetup/cryptsetup/blob/master/docs/Keyring.txt)
