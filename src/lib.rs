#![no_std]

const SHIFT_4K: u8 = 12;
const SHIFT_2M: u8 = 21;
const SHIFT_1G: u8 = 30;
const SHIFT_512G: u8 = 39;

const KiB: usize = 1024;
const MiB: usize = 0x100000;
const GiB: usize = 0x40000000;

const PAGE_4K: usize = 2 * KiB;
const PAGE_2M: usize = 2 * MiB;