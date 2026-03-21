use crate::SimpleDecoderError;
use crate::decoders::AudioDecoder;
use crate::decoders::aac::AACDecoder;
use crate::decoders::flac::FLACDecoder;
use crate::decoders::mp3::MP3Decoder;
use crate::decoders::oggopus::OGGOpusDecoder;
use crate::decoders::oggvorbis::OGGVorbisDecoder;
use crate::decoders::wav::WAVDecoder;
use crate::types::AudioTrack;
use std::io::Cursor;
use symphonia::core::codecs::{
    CODEC_TYPE_AAC, CODEC_TYPE_FLAC, CODEC_TYPE_MP3, CODEC_TYPE_OPUS, CODEC_TYPE_PCM_S16LE,
    CODEC_TYPE_VORBIS,
};
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
            CODEC_TYPE_AAC => {
                let aac = AACDecoder::new();
                Ok(aac.decode(data)?)
            }
            CODEC_TYPE_MP3 => {
                let mp3 = MP3Decoder::new();
                Ok(mp3.decode(data)?)
            }
            CODEC_TYPE_OPUS => {
                let opus = OGGOpusDecoder::new();
                Ok(opus.decode(data)?)
            }
            CODEC_TYPE_VORBIS => {
                let vorbis = OGGVorbisDecoder::new();
                Ok(vorbis.decode(data)?)
            }
            CODEC_TYPE_PCM_S16LE => {
                let wav = WAVDecoder::new();
                Ok(wav.decode(data)?)
            }
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
