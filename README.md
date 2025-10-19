# AX/2
AX/2 is a toy operating system project - a kernel and small userspace built in Rust for modern AMD64
computers. It is not a UNIX, and does not attempt to be compatible with existing software.

### Features

### External components
The project is not strictly NIH, but I do aim to build my own versions of things in-tree. Small
libraries, especially in tools like `xtask`, are pulled in relatively freely.

Major external components include:
- [`uefi-rs`](https://github.com/rust-osdev/uefi-rs) - Provides Rust bindings to UEFI
- [`acpi`](https://github.com/rust-osdev/acpi) - A pure-Rust ACPI implementation (maintained primarily be me)

### License
AX/2 is licensed under the [Mozilla Public License 2.0 (MPLv2)](https://www.mozilla.org/en-US/MPL/2.0/).
