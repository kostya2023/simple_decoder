use crate::SimpleDecoderError;
use crate::decoders::AudioDecoder;
use crate::types::{AudioFormat, AudioTrack};
use opus::{Channels, Decoder};
use std::io::Cursor;
use symphonia::core::codecs::CODEC_TYPE_OPUS;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct OGGOpusDecoder;

impl AudioDecoder for OGGOpusDecoder {
    fn new() -> Self {
        Self
    }

    fn decode(&self, data: &[u8]) -> std::result::Result<AudioTrack, SimpleDecoderError> {
        let mss = MediaSourceStream::new(Box::new(Cursor::new(data.to_vec())), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("opus");

        let mut probed = symphonia::default::get_probe()
            .format(
                &hint,
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            )
            .map_err(SimpleDecoderError::DecoderError)?;

        let track = probed
            .format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec == CODEC_TYPE_OPUS)
            .ok_or(Error::DecodeError("No track found"))
            .map_err(SimpleDecoderError::DecoderError)?;

        let codec_params = &track.codec_params;
        let sample_rate = codec_params
            .sample_rate
            .ok_or(Error::DecodeError("No sample rate found"))
            .map_err(SimpleDecoderError::DecoderError)? as u64;
        let channels = codec_params
            .channels
            .ok_or(Error::DecodeError("No channels found"))
            .map_err(SimpleDecoderError::DecoderError)?
            .count() as u16;
        let track_id = track.id;

        let decoder_channel = match channels {
            1 => Ok(Channels::Mono),
            2 => Ok(Channels::Stereo),
            _ => Err(Error::DecodeError("Unsupported channels count")),
        }
        .map_err(SimpleDecoderError::DecoderError)?;

        let mut decoder = Decoder::new(sample_rate as u32, decoder_channel).map_err(|_| {
            SimpleDecoderError::ResampleError("Unable to create opus decoder".to_string())
        })?;

        let mut pcm: Vec<f32> = vec![];

        loop {
            let packet = match probed.format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(_)) => break,
                Err(e) => return Err(SimpleDecoderError::DecoderError(e)),
            };

            if packet.track_id() != track_id {
                continue;
            };

            let mut output_buff = vec![0.0f32; 5760 * channels as usize];

            let decoded_samples = decoder
                .decode_float(&packet.data, &mut output_buff, false)
                .map_err(|_| {
                    SimpleDecoderError::ResampleError("Unable to decode data".to_string())
                })?;

            pcm.extend_from_slice(&output_buff[..decoded_samples * channels as usize]);
        }

        Ok(AudioTrack {
            format: AudioFormat::OGGOpus,
            channels,
            sample_rate,
            pcm,
        })
    }
}
