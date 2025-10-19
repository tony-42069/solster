#![no_std]

pub mod state;
pub mod instructions;
pub mod matching;

#[cfg(feature = "bpf-entrypoint")]
mod entrypoint;

pub use state::*;
pub use instructions::*;
pub use matching::*;

pinocchio_pubkey::declare_id!("SlabXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
