#[cfg(feature = "std")]
use thiserror::Error;
#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum XRPLMultisignException {
    #[error("No signers set in the transaction. Use `sign` function with `multisign = true`.")]
    NoSigners,
}
