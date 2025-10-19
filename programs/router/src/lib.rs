#![no_std]

pub mod state;
pub mod instructions;

#[cfg(feature = "bpf-entrypoint")]
mod entrypoint;

pub use state::*;
pub use instructions::*;

pinocchio_pubkey::declare_id!("RouterXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
