//! General XRPL Address Codec Exception.

#[cfg(feature = "std")]
use thiserror::Error;
#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum XRPLAddressCodecException {
    #[error("Invalid XAddress prefix")]
    InvalidXAddressPrefix,
    #[error("Invalid XAddress zero tag")]
    InvalidXAddressZeroNoTag,
    #[error("Invalid XAddress zero remain")]
    InvalidXAddressZeroRemain,
    #[error("Invalid classic address length (length: {length})")]
    InvalidCAddressIdLength { length: usize },
    #[error("Invalid classic address tag")]
    InvalidCAddressTag,
    #[error("Invalid seed prefix encoding type")]
    InvalidSeedPrefixEncodingType,
    #[error("Invalid encoding prefix length")]
    InvalidEncodingPrefixLength,
    #[error("Invalid classic address value")]
    InvalidClassicAddressValue,
    #[error("Unsupported XAddress")]
    UnsupportedXAddress,
    #[error("Unknown seed encoding")]
    UnknownSeedEncoding,
    #[error("Unknown payload lenght (expected: {expected}, found: {found})")]
    UnexpectedPayloadLength { expected: usize, found: usize },
    #[error("Base58 decode error: {0}")]
    Base58DecodeError(#[from] bs58::decode::Error),
    #[error("Vec resize error: expected length {expected}, got {actual}")]
    VecResizeError { expected: usize, actual: usize },
}
