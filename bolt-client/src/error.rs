use bolt_proto::version::*;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type ConnectionResult<T> = std::result::Result<T, ConnectionError>;

// TODO: Break into more specific error types
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "tokio-stream")]
    #[error("invalid DNS name: {0}")]
    InvalidDNSName(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("handshake with server failed for versions [{}]", format_versions(.0))]
    HandshakeFailed([u32; 4]),
    #[error("unsupported operation for client with version = {}", format_version(*.0))]
    UnsupportedOperation(u32),
    #[error("unsupported operation for server in {state:?} state: {message:?}")]
    InvalidState {
        state: bolt_proto::ServerState,
        message: bolt_proto::Message,
    },
    #[error(
        "server gave unexpected response while in {state:?} state.
request: {request:?}
response: {response:?}"
    )]
    InvalidResponse {
        state: bolt_proto::ServerState,
        request: Option<bolt_proto::Message>,
        response: bolt_proto::Message,
    },
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
    #[error(transparent)]
    ProtocolError(#[from] bolt_proto::error::Error),
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("handshake with server failed for versions [{}]", format_versions(.0))]
    HandshakeFailed([u32; 4]),
    #[error("invalid DNS name: {0}")]
    InvalidDNSName(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

fn format_version(version: u32) -> String {
    match version {
        V1_0 => String::from("1.0"),
        V2_0 => String::from("2.0"),
        V3_0 => String::from("3.0"),
        V4_0 => String::from("4.0"),
        V4_1 => String::from("4.1"),
        _ => format!("{:#x}", version),
    }
}

fn format_versions(versions: &[u32]) -> String {
    versions
        .iter()
        .map(|&v| format_version(v))
        .collect::<Vec<String>>()
        .join(", ")
}
