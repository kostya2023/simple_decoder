use std::io::Cursor;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_FLAC, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::SimpleDecoderError;
use crate::decoders::AudioDecoder;
use crate::types::{AudioFormat, AudioTrack};

pub struct FLACDecoder;

impl AudioDecoder for FLACDecoder {
    fn new() -> Self {
        Self
    }

    fn decode(&self, data: &[u8]) -> Result<AudioTrack, SimpleDecoderError> {
        let mss = MediaSourceStream::new(Box::new(Cursor::new(data.to_vec())), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("wav");

        let mut probed = symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let (track_id, codec_params) = {
            let track = probed
                .format
                .tracks()
                .iter()
                .find(|t| t.codec_params.codec == CODEC_TYPE_FLAC)
                .ok_or(Error::DecodeError("No track found"))?;

            (track.id, track.codec_params.clone())
        };

        let mut decoder =
            symphonia::default::get_codecs().make(&codec_params, &DecoderOptions::default())?;

        let mut pcm: Vec<f32> = vec![];

        let mut sample_rate = codec_params.sample_rate.unwrap_or(48000) as u64;
        let mut channels = codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);

        let mut sample_buf = None;

        loop {
            let packet = match probed.format.next_packet() {
                Ok(packet) => packet,
                Err(symphonia::core::errors::Error::IoError(_)) => break,
                Err(e) => return Err(e.into()),
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = *decoded.spec();

                    sample_rate = spec.rate as u64;
                    channels = spec.channels.count() as u16;

                    let buf = sample_buf.get_or_insert_with(|| {
                        SampleBuffer::<f32>::new(decoded.capacity() as u64, spec)
                    });

                    buf.copy_interleaved_ref(decoded);
                    pcm.extend_from_slice(buf.samples());
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                Err(e) => return Err(e.into()),
            }
        }

        Ok(AudioTrack {
            format: AudioFormat::FLAC,
            channels,
            sample_rate,
            pcm,
        })
    }
}
