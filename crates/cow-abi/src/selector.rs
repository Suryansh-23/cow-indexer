use crate::*;
use alloy::primitives::B256;

#[derive(Clone, Debug)]
pub struct Selector {
    pub trade: B256,
    pub interaction: B256,
    pub settlement: B256,
    pub order_invalidated: B256,
    pub pre_signature: B256,
}

impl Default for Selector {
    fn default() -> Self {
        Self {
            trade: Trade::SIGNATURE_HASH,
            interaction: Interaction::SIGNATURE_HASH,
            settlement: Settlement::SIGNATURE_HASH,
            order_invalidated: OrderInvalidated::SIGNATURE_HASH,
            pre_signature: PreSignature::SIGNATURE_HASH,
        }
    }
}
