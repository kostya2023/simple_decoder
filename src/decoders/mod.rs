use crate::{SimpleDecoderError, types::AudioTrack};

pub trait AudioDecoder {
    fn new() -> Self;
    fn decode(&self, data: &[u8]) -> Result<AudioTrack, SimpleDecoderError>;
}

pub mod aac;
pub mod flac;
pub mod mp3;
pub mod oggopus;
pub mod oggvorbis;
pub mod simple;
pub mod wav;
