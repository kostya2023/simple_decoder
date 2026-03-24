use crate::SimpleDecoderError;
use crate::decoders::AudioDecoder;

#[cfg(feature = "aac")]
use crate::decoders::aac::AACDecoder;
#[cfg(feature = "aac")]
use symphonia::core::codecs::CODEC_TYPE_AAC;

#[cfg(feature = "flac")]
use crate::decoders::flac::FLACDecoder;
#[cfg(feature = "flac")]
use symphonia::core::codecs::CODEC_TYPE_FLAC;

#[cfg(feature = "mp3")]
use crate::decoders::mp3::MP3Decoder;
#[cfg(feature = "mp3")]
use symphonia::core::codecs::CODEC_TYPE_MP3;

#[cfg(feature = "oggopus")]
use crate::decoders::oggopus::OGGOpusDecoder;
#[cfg(feature = "oggopus")]
use symphonia::core::codecs::CODEC_TYPE_OPUS;

#[cfg(feature = "oggvorbis")]
use crate::decoders::oggvorbis::OGGVorbisDecoder;
#[cfg(feature = "oggvorbis")]
use symphonia::core::codecs::CODEC_TYPE_VORBIS;

#[cfg(feature = "wav")]
use crate::decoders::wav::WAVDecoder;
#[cfg(feature = "wav")]
use symphonia::core::codecs::CODEC_TYPE_PCM_S16LE;

use crate::types::AudioTrack;
use std::io::Cursor;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct SimpleDecoder;

impl AudioDecoder for SimpleDecoder {
    fn new() -> Self {
        Self
    }

    fn decode(&self, data: &[u8]) -> Result<AudioTrack, SimpleDecoderError> {
        let mss = MediaSourceStream::new(Box::new(Cursor::new(data.to_vec())), Default::default());
        let hint = Hint::new();

        let probed = symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let codec_id = probed
            .format
            .tracks()
            .iter()
            .next()
            .map(|t| t.codec_params.codec)
            .ok_or(Error::DecodeError("No track found"))?;

        let result = match codec_id {
            #[cfg(feature = "aac")]
            CODEC_TYPE_AAC => {
                let aac = AACDecoder::new();
                Ok(aac.decode(data)?)
            }
            #[cfg(feature = "mp3")]
            CODEC_TYPE_MP3 => {
                let mp3 = MP3Decoder::new();
                Ok(mp3.decode(data)?)
            }
            #[cfg(feature = "oggopus")]
            CODEC_TYPE_OPUS => {
                let opus = OGGOpusDecoder::new();
                Ok(opus.decode(data)?)
            }
            #[cfg(feature = "oggvorbis")]
            CODEC_TYPE_VORBIS => {
                let vorbis = OGGVorbisDecoder::new();
                Ok(vorbis.decode(data)?)
            }
            #[cfg(feature = "wav")]
            CODEC_TYPE_PCM_S16LE => {
                let wav = WAVDecoder::new();
                Ok(wav.decode(data)?)
            }
            #[cfg(feature = "flac")]
            CODEC_TYPE_FLAC => {
                let flac = FLACDecoder::new();
                Ok(flac.decode(data)?)
            }
            _ => Err(SimpleDecoderError::DecoderError(Error::Unsupported(
                "Unsupported format",
            ))),
        }?;
        Ok(result)
    }
}
