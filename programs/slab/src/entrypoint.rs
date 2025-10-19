//! Slab program entrypoint

use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    pubkey::Pubkey,
    ProgramResult,
};

// Set up panic handler and allocator for BPF builds
#[cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]
use core::panic::PanicInfo;

#[cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Percolator Slab");
    Ok(())
}
