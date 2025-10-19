/// Router instructions (stub implementations)

use percolator_common::*;

/// Instruction discriminator
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouterInstruction {
    /// Initialize router
    Initialize = 0,
    /// Deposit collateral
    Deposit = 1,
    /// Withdraw collateral
    Withdraw = 2,
    /// Multi-slab reserve orchestration
    MultiReserve = 3,
    /// Multi-slab commit orchestration
    MultiCommit = 4,
    /// Liquidation coordinator
    Liquidate = 5,
}

pub fn process_instruction(
    _instruction: RouterInstruction,
    _data: &[u8],
) -> Result<(), PercolatorError> {
    Ok(())
}
