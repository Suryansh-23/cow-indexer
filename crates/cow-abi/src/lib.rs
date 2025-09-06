use alloy::sol;
use alloy::sol_types::SolEvent;

pub mod network;
pub mod selector;

sol! {
    /// @dev Event emitted for each executed trade.
    event Trade(
        address indexed owner,
        address sellToken,
        address buyToken,
        uint256 sellAmount,
        uint256 buyAmount,
        uint256 feeAmount,
        bytes orderUid
    );

    /// @dev Event emitted for each executed interaction.
    ///
    /// For gas efficiency, only the interaction calldata selector (first 4
    /// bytes) is included in the event. For interactions without calldata or
    /// whose calldata is shorter than 4 bytes, the selector will be `0`.
    event Interaction(address indexed target, uint256 value, bytes4 selector);

    /// @dev Event emitted when a settlement completes
    event Settlement(address indexed solver);

    /// @dev Event emitted when an order is invalidated.
    event OrderInvalidated(address indexed owner, bytes orderUid);

    /// @dev Event that is emitted when an account either pre-signs an order or
    /// revokes an existing pre-signature.
    event PreSignature(address indexed owner, bytes orderUid, bool signed);
}
