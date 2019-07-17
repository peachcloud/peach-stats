use std::error;

use jsonrpc_core::{types::error::Error, ErrorCode};
use probes::ProbeError;
use serde_json::Error as SerdeError;
use snafu::Snafu;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum StatError {
    #[snafu(display("Failed to retrieve measurement of CPU statistics"))]
    GetCpuStat,

    #[snafu(display("Failed to retrieve measurement of CPU statistics: {}", source))]
    ReadCpuStat { source: ProbeError },

    #[snafu(display("JSON serialization failed: {}", source))]
    SerdeSerialize { source: SerdeError },
}

impl From<StatError> for Error {
    fn from(err: StatError) -> Self {
        match &err {
            StatError::GetCpuStat => Error {
                code: ErrorCode::ServerError(-32000),
                message: "Failed to retrieve measurement of CPU statistics".to_string(),
                data: None,
            },
            StatError::ReadCpuStat { source } => Error {
                code: ErrorCode::ServerError(-32001),
                message: format!(
                    "Failed to retrieve measurement of CPU statistics: {}",
                    source
                ),
                data: None,
            },
            StatError::SerdeSerialize { source } => Error {
                code: ErrorCode::ServerError(-32002),
                message: format!("JSON serialization failed: {}", source),
                data: None,
            },
        }
    }
}
