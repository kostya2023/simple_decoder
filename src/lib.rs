#![allow(clippy::needless_pub_self)]

pub mod decoders;
pub(crate) mod types;
pub(self) use symphonia::core::errors::Error;
pub(self) use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimpleDecoderError {
    #[error("Decode error: {0}")]
    DecoderError(#[from] Error),

    #[error("Resample error: {0}")]
    ResampleError(String),
}

pub use crate::decoders::simple::SimpleDecoder;
pub mod audio {
    pub use crate::types::{AudioFormat, AudioTrack};
}
