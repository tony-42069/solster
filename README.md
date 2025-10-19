# Percolator

A sharded perpetual exchange protocol for Solana, implementing the design from `plan.md`.

## Architecture

Percolator consists of two main on-chain programs:

### 1. Router Program
The global coordinator managing collateral, portfolio margin, and cross-slab routing.

**Program ID:** `RoutR1VdCpHqj89WEMJhb6TkGT9cPfr1rVjhM3e2YQr`

**State structures:**
- `Vault` - Collateral custody per asset mint
- `Escrow` - Per (user, slab, mint) pledges with anti-replay nonces
- `Cap` (Capability) - Time-limited, scoped debit authorization tokens (max 2 minutes TTL)
- `Portfolio` - Cross-margin tracking with exposure aggregation across slabs
- `SlabRegistry` - Governance-controlled registry with version validation

### 2. Slab Program
LP-run perp engines with 10 MB state budget, fully self-contained matching and settlement.

**Program ID:** `SLabZ6PsDLh2X6HzEoqxFDMqCVcJXDKCNEYuPzUvGPk`

**State structures:**
- `SlabHeader` - Metadata, risk params, anti-toxicity settings
- `Instrument` - Contract specs, oracle prices, funding rates, book heads
- `Order` - Price-time sorted orders with reservation tracking
- `Position` - User positions with VWAP entry prices
- `Reservation` - Reserve-commit two-phase execution state
- `Slice` - Sub-order fragments locked during reservation
- `Trade` - Ring buffer of executed trades
- `AggressorEntry` - Anti-sandwich tracking per batch

## Key Features Implemented

### ✅ Memory Management
- **10 MB budget** strictly enforced at compile time
- O(1) freelist-based allocation for all pools
- Zero allocations after initialization
- Pool sizes (tuned to fit within 10 MB):
  - Accounts: 5,000
  - Orders: 30,000
  - Positions: 30,000
  - Reservations: 4,000
  - Slices: 16,000
  - Trades: 10,000 (ring buffer)
  - Instruments: 32
  - DLP accounts: 100
  - Aggressor entries: 4,000

### ✅ Matching Engine
- **Price-time priority** with strict FIFO at same price level
- **Reserve operation**: Walk book, lock slices, calculate VWAP/worst price
- **Commit operation**: Execute at captured maker prices
- **Cancel operation**: Release reservations
- **Pending queue promotion**: Non-DLP orders wait one batch epoch

### ✅ Risk Management
- **Local (slab) margin**: IM/MM calculated per position
- **Global (router) margin**: Cross-slab portfolio netting
- Equity calculation with unrealized PnL and funding payments
- Pre-trade margin checks
- Liquidation detection

### ✅ Capability Security
- Time-limited caps (max 2 minutes TTL)
- Scoped to (user, slab, mint) triplet
- Anti-replay with nonces
- Remaining amount tracking
- Automatic expiry checks

### ✅ Fixed-Point Math
- 6-decimal precision for prices
- VWAP calculations
- PnL computation
- Funding payment tracking
- Margin calculations in basis points

### ✅ Anti-Toxicity Infrastructure
- Batch windows (`batch_ms`)
- Delayed maker posting (pending → live promotion)
- JIT penalty detection
- Kill band parameters
- Freeze levels configuration
- Aggressor roundtrip guard (ARG) data structures

## Current Status

### Working
- ✅ Core data structures
- ✅ Memory pools with freelists
- ✅ Order book management
- ✅ Reserve operation
- ✅ Risk calculations
- ✅ Capability system
- ✅ Fixed-point math utilities
- ✅ Compile-time size constraints

### In Progress
- ⏳ Commit operation (has borrow checker issues to resolve)
- ⏳ Position management (needs refactoring for Rust borrow rules)

### TODO
- ❌ Anti-toxicity mechanism integration
- ❌ Funding rate updates
- ❌ Liquidation execution
- ❌ Router orchestration (multi-slab reserve/commit)
- ❌ Instruction parsing and validation
- ❌ PDA derivations and account initialization
- ❌ Unit tests
- ❌ Integration tests with Surfpool
- ❌ Property-based tests for invariants
- ❌ Build for BPF target

## Technical Details

### Technology Stack
- **Framework**: [Pinocchio](https://github.com/anza-xyz/pinocchio) v0.9.2 - Zero-dependency Solana SDK
- **Testing**: [Surfpool](https://github.com/txtx/surfpool) - Local Solana test validator with mainnet state
- **Language**: Rust (no_std, zero allocations)

### Design Invariants (from plan.md)

**Safety:**
1. Slabs cannot access Router vaults directly
2. Slabs can only debit via unexpired, correctly scoped Caps
3. Total debits ≤ min(cap.remaining, escrow.balance)
4. No cross-contamination: slab cannot move funds for (user', slab') ≠ (user, slab)

**Matching:**
1. Price-time priority strictly maintained
2. Reserved qty ≤ available qty always
3. Book links acyclic and consistent
4. Pending orders never match before promotion

**Risk:**
1. IM monotone: increasing exposure increases margin
2. Portfolio IM ≤ Σ slab IMs (convexity not double-counted)
3. Liquidation triggers only when equity < MM

**Anti-Toxicity:**
1. Kill band: reject if mark moved > threshold
2. JIT penalty: DLP orders posted after batch_open get no rebate
3. ARG: roundtrip trades within batch are taxed/clipped

## Building

```bash
# Build all programs (libraries)
cargo build

# Build for Solana BPF target (requires solana toolchain)
# TODO: Add build-sbf support

# Run tests
cargo test
```

## Known Issues

1. **Borrow Checker Errors**: The commit module has several places where we need mutable access to different parts of `SlabState` simultaneously. This needs refactoring to use split borrows or interior mutability patterns.

2. **Vec Usage**: The `promote_pending` function temporarily uses `Vec` for collecting orders to promote. In a true no_std/no_alloc environment, this needs to be rewritten with a fixed buffer or multiple passes.

3. **Missing Tests**: Core logic has inline tests, but comprehensive unit/integration/property tests are needed.

4. **BPF Build**: Programs don't yet build for the Solana BPF target. Need to configure proper build scripts.

## Next Steps

1. **Fix Borrow Checker Issues**: Refactor commit/position management to avoid overlapping borrows
2. **Complete Instruction Handlers**: Wire up actual instruction parsing and account validation
3. **Add PDA Helpers**: Implement seed derivations for all account types
4. **Integration Tests**: Set up Surfpool-based test scenarios
5. **Property Tests**: Verify invariants hold under random operations
6. **BPF Build**: Configure Solana toolchain and build scripts
7. **Benchmarks**: Measure CU consumption and latency targets

## References

- [Plan Document](./plan.md) - Full protocol specification
- [Pinocchio Docs](https://docs.rs/pinocchio/)
- [Surfpool](https://github.com/txtx/surfpool)

## License

Apache-2.0

---

**Status**: Implementation in progress - Core infrastructure complete, matching engine functional but needs borrow checker fixes before testing.
