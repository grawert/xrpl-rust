#[cfg(feature = "std")]
use thiserror::Error;
#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XRPLJsonRpcException {
    #[error("Reqwless error: {0:?}")]
    ReqwlessError(reqwless::Error),
    #[cfg(feature = "std")]
    #[error("Reqwest error: {0:?}")]
    ReqwestError(#[from] reqwest::Error),
}

impl From<reqwless::Error> for XRPLJsonRpcException {
    fn from(err: reqwless::Error) -> Self {
        XRPLJsonRpcException::ReqwlessError(err)
    }
}
