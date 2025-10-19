//! Reserve instruction - stub implementation

use crate::matching::reserve as reserve_fn;
use crate::state::SlabState;
use percolator_common::*;

pub fn process_reserve(
    _slab: &mut SlabState,
    _account_idx: u32,
    _instrument_idx: u16,
    _side: Side,
    _qty: u64,
    _limit_px: u64,
    _ttl_ms: u64,
    _commitment_hash: [u8; 32],
    _route_id: u64,
) -> Result<(), PercolatorError> {
    // Implementation delegated to matching::reserve
    Ok(())
}
