use crate::{SimpleDecoderError, types::AudioTrack};

pub trait AudioDecoder {
    fn new() -> Self;
    fn decode(&self, data: &[u8]) -> Result<AudioTrack, SimpleDecoderError>;
}

#[cfg(feature = "aac")]
pub mod aac;
#[cfg(feature = "flac")]
pub mod flac;
#[cfg(feature = "mp3")]
pub mod mp3;
#[cfg(feature = "oggopus")]
pub mod oggopus;
#[cfg(feature = "oggvorbis")]
pub mod oggvorbis;
pub mod simple;
#[cfg(feature = "wav")]
pub mod wav;
