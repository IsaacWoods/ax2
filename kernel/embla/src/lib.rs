//! `embla` is a library of useful data structures for use in the kernelspace and services of AX/2

#![cfg_attr(not(test), no_std)]
#![feature(pattern, trim_prefix_suffix)]

pub mod bit_ops;
pub mod cmdline;
pub mod sync;

/// Align `x` to `align`, where `align` is a power-of-2 or `0`, where the result will be `<=x`.
pub fn align_down(x: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        /*
         * E.g.
         *      align       =   0b00001000
         *      align-1     =   0b00000111
         *      !(align-1)  =   0b11111000
         *                             ^^^ Masks the address to the value below it with the
         *                                 correct alignment
         */
        x & !(align - 1)
    } else {
        assert!(align == 0);
        x
    }
}

/// Align `x` to `align`, where `align` is a power-of-2 or `0`, where the result will be `>=x`.
pub fn align_up(x: usize, align: usize) -> usize {
    align_down(x + align - 1, align)
}
