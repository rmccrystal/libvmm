use core::arch::asm;
use crate::{AlignedAddress, SHIFT_4K};

pub mod exits;
pub mod msr;
pub mod vmcs;
pub mod vmcs_validator;