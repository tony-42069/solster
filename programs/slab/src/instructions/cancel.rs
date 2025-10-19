//! Cancel instruction - stub implementation

use crate::state::SlabState;
use percolator_common::*;

pub fn process_cancel(
    _slab: &mut SlabState,
    _hold_id: u64,
) -> Result<(), PercolatorError> {
    // Implementation delegated to matching::cancel
    Ok(())
}
