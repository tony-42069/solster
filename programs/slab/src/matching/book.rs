//! Order book management with price-time priority

use crate::state::SlabState;
use percolator_common::*;

/// Insert order into book maintaining price-time priority
pub fn insert_order(
    slab: &mut SlabState,
    instrument_idx: u16,
    order_idx: u32,
    side: Side,
    price: u64,
    state: OrderState,
) -> Result<(), PercolatorError> {
    let instrument = slab
        .get_instrument_mut(instrument_idx)
        .ok_or(PercolatorError::InvalidInstrument)?;

    let (head, pending_head) = match side {
        Side::Buy => (&mut instrument.bids_head, &mut instrument.bids_pending_head),
        Side::Sell => (&mut instrument.asks_head, &mut instrument.asks_pending_head),
    };

    let target_head = match state {
        OrderState::LIVE => head,
        OrderState::PENDING => pending_head,
    };

    // If empty list, set as head
    if *target_head == u32::MAX {
        *target_head = order_idx;
        if let Some(order) = slab.orders.get_mut(order_idx) {
            order.next = u32::MAX;
            order.prev = u32::MAX;
        }
        slab.header.increment_book_seqno();
        return Ok(());
    }

    // Find insertion point maintaining price-time priority
    let mut curr_idx = *target_head;
    let mut prev_idx = u32::MAX;

    while curr_idx != u32::MAX {
        let curr_order = slab
            .orders
            .get(curr_idx)
            .ok_or(PercolatorError::OrderNotFound)?;

        // Price-time priority:
        // Buy: higher price first, then earlier order_id
        // Sell: lower price first, then earlier order_id
        let should_insert_before = match side {
            Side::Buy => {
                price > curr_order.price
                    || (price == curr_order.price
                        && slab.orders.get(order_idx).unwrap().order_id < curr_order.order_id)
            }
            Side::Sell => {
                price < curr_order.price
                    || (price == curr_order.price
                        && slab.orders.get(order_idx).unwrap().order_id < curr_order.order_id)
            }
        };

        if should_insert_before {
            break;
        }

        prev_idx = curr_idx;
        curr_idx = curr_order.next;
    }

    // Insert order
    if let Some(order) = slab.orders.get_mut(order_idx) {
        order.next = curr_idx;
        order.prev = prev_idx;
    }

    // Update prev's next
    if prev_idx == u32::MAX {
        // Inserting at head
        *target_head = order_idx;
    } else if let Some(prev_order) = slab.orders.get_mut(prev_idx) {
        prev_order.next = order_idx;
    }

    // Update next's prev
    if curr_idx != u32::MAX {
        if let Some(curr_order) = slab.orders.get_mut(curr_idx) {
            curr_order.prev = order_idx;
        }
    }

    slab.header.increment_book_seqno();
    Ok(())
}

/// Remove order from book
pub fn remove_order(
    slab: &mut SlabState,
    instrument_idx: u16,
    order_idx: u32,
) -> Result<(), PercolatorError> {
    let order = slab
        .orders
        .get(order_idx)
        .ok_or(PercolatorError::OrderNotFound)?;

    let side = order.side;
    let state = order.state;
    let prev = order.prev;
    let next = order.next;

    let instrument = slab
        .get_instrument_mut(instrument_idx)
        .ok_or(PercolatorError::InvalidInstrument)?;

    let target_head = match (side, state) {
        (Side::Buy, OrderState::LIVE) => &mut instrument.bids_head,
        (Side::Buy, OrderState::PENDING) => &mut instrument.bids_pending_head,
        (Side::Sell, OrderState::LIVE) => &mut instrument.asks_head,
        (Side::Sell, OrderState::PENDING) => &mut instrument.asks_pending_head,
    };

    // Update links
    if prev == u32::MAX {
        // Removing head
        *target_head = next;
    } else if let Some(prev_order) = slab.orders.get_mut(prev) {
        prev_order.next = next;
    }

    if next != u32::MAX {
        if let Some(next_order) = slab.orders.get_mut(next) {
            next_order.prev = prev;
        }
    }

    slab.header.increment_book_seqno();
    Ok(())
}

/// Promote pending orders to live book
pub fn promote_pending(
    slab: &mut SlabState,
    instrument_idx: u16,
    epoch: u16,
) -> Result<(), PercolatorError> {
    let instrument = slab
        .get_instrument_mut(instrument_idx)
        .ok_or(PercolatorError::InvalidInstrument)?;

    // Promote bids
    promote_side(slab, instrument_idx, Side::Buy, epoch)?;

    // Promote asks
    promote_side(slab, instrument_idx, Side::Sell, epoch)?;

    Ok(())
}

/// Promote pending orders for one side
fn promote_side(
    slab: &mut SlabState,
    instrument_idx: u16,
    side: Side,
    epoch: u16,
) -> Result<(), PercolatorError> {
    let instrument = slab
        .get_instrument(instrument_idx)
        .ok_or(PercolatorError::InvalidInstrument)?;

    let pending_head = match side {
        Side::Buy => instrument.bids_pending_head,
        Side::Sell => instrument.asks_pending_head,
    };

    let mut curr_idx = pending_head;
    let mut to_promote = Vec::new();

    // Collect orders to promote (eligible_epoch <= current epoch)
    while curr_idx != u32::MAX {
        if let Some(order) = slab.orders.get(curr_idx) {
            if order.eligible_epoch <= epoch {
                to_promote.push((curr_idx, order.price));
            }
            curr_idx = order.next;
        } else {
            break;
        }
    }

    // Promote each order
    for (order_idx, price) in to_promote {
        // Remove from pending
        remove_order(slab, instrument_idx, order_idx)?;

        // Update state to LIVE
        if let Some(order) = slab.orders.get_mut(order_idx) {
            order.state = OrderState::LIVE;
        }

        // Insert into live book
        insert_order(slab, instrument_idx, order_idx, side, price, OrderState::LIVE)?;
    }

    Ok(())
}

/// Get best bid/ask for instrument
pub fn get_best_prices(slab: &SlabState, instrument_idx: u16) -> Result<(Option<u64>, Option<u64>), PercolatorError> {
    let instrument = slab
        .get_instrument(instrument_idx)
        .ok_or(PercolatorError::InvalidInstrument)?;

    let best_bid = if instrument.bids_head != u32::MAX {
        slab.orders.get(instrument.bids_head).map(|o| o.price)
    } else {
        None
    };

    let best_ask = if instrument.asks_head != u32::MAX {
        slab.orders.get(instrument.asks_head).map(|o| o.price)
    } else {
        None
    };

    Ok((best_bid, best_ask))
}

// Note: Vec is used here for simplicity in the promotion logic.
// In a no_std environment with no_allocator, this would need to be rewritten
// to use a fixed-size buffer or multiple passes. For now, keeping it simple.
#[cfg(not(feature = "no_std"))]
extern crate alloc;
#[cfg(not(feature = "no_std"))]
use alloc::vec::Vec;

#[cfg(feature = "no_std")]
compile_error!("Promotion logic needs to be rewritten for no_std without allocation");
