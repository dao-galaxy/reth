#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
//! Implementation of the `eth` wire protocol.

pub mod builder;
pub mod capability;
mod disconnect;
pub mod error;
mod ethstream;
mod p2pstream;
mod pinger;
mod hello;
pub use builder::*;
pub mod types;
pub use types::*;

#[cfg(test)]
pub use tokio_util::codec::{
    LengthDelimitedCodec as PassthroughCodec, LengthDelimitedCodecError as PassthroughCodecError,
};

pub use crate::{
    disconnect::DisconnectReason,
    hello::HelloMessage,
    ethstream::{EthStream, UnauthedEthStream, MAX_MESSAGE_SIZE},
    p2pstream::{P2PStream, ProtocolVersion, UnauthedP2PStream},
};
