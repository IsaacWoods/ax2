//! `embla` is a library of useful data structures for use in the kernelspace and services of AX/2

#![cfg_attr(not(test), no_std)]
#![feature(pattern, trim_prefix_suffix)]

pub mod bit_ops;
pub mod cmdline;
pub mod sync;
