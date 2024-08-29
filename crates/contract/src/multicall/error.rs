use alloy_transport::TransportError;


#[derive(Debug, thiserror::Error)]
/// Errors that can occur when interacting with the Multicall contract
#[allow(missing_docs)]
pub enum MultiCallError {
    #[error("A call with no target address was attempted to be added to the multicall")]
    MissingTargetAddress,

    #[error("The multicall contract is not deployed on the current chain")]
    ChainNotSupported,

    #[error("Decoding Failed: {0}")]
    DecoderError(#[from] alloy_sol_types::Error),

    #[error("Contract Error: {0}")]
    ContractError(#[from] crate::Error),

    #[error("Transport Error: {0}")]
    TransportError(#[from] TransportError),
}
