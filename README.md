[![Build Status](https://travis-ci.org/gdamjan/fb-ask-pass-rs.svg?branch=master)](https://travis-ci.org/gdamjan/fb-ask-pass-rs)


Primary usage: run in the initramfs to ask for the LUKS passphrase, while showing the firmware picture.

# FAQ

## Why Rust?

- It's a learning experience
- single static binary, depends only on glibc

## How does it work?

- the program reads the firmware image provided by ACPI 5.0 from `/sys/firmware/acpi/bgrt/*` and displays it on the
  framebuffer at the same position (`xoffset`, `yoffset`).
  - afaik, UEFI needs to be enabled. probably quick boot, and full resolution booting too.
