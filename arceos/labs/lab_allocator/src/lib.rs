//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

mod slab;
mod tlsf;

pub type LabByteAllocator = slab::SlabByteAllocator;
