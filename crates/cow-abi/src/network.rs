use alloy::primitives::{Address, ChainId, address};

#[derive(Clone, Debug)]
pub struct Network {
    pub chain_id: ChainId,
    pub address: Address,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            chain_id: 1,
            address: address!("0x9008D19f58AAbD9eD0D60971565AA8510560ab41"),
        }
    }
}
