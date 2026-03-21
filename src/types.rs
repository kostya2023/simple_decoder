use crate::SimpleDecoderError;
use audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Fft, FixedSync, Indexing, Resampler};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioFormat {
    MP3,
    OGGOpus,
    OGGVorbis,
    AAC,
    WAV,
    FLAC,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AudioTrack {
    pub format: AudioFormat,
    pub channels: u16,
    pub sample_rate: u64,
    pub pcm: Vec<f32>,
}

impl AudioTrack {
    pub fn resample(&self, target_rate: u64) -> Result<Self, SimpleDecoderError> {
        if self.sample_rate == target_rate {
            return Ok(self.clone());
        }

        let channels = self.channels as usize;

        let mut resampler = Fft::<f32>::new(
            self.sample_rate as usize,
            target_rate as usize,
            1024,
            2,
            channels,
            FixedSync::Both,
        )
        .map_err(|e| SimpleDecoderError::ResampleError(e.to_string()))?;

        let nbr = self.pcm.len() / channels;

        let input_adapter = InterleavedSlice::new(&self.pcm, channels, nbr)
            .map_err(|e| SimpleDecoderError::ResampleError(e.to_string()))?;

        let out_len_needed = resampler.process_all_needed_output_len(nbr);
        let mut out_pcm = vec![0.0f32; out_len_needed * channels];

        let mut output_adatper = InterleavedSlice::new_mut(&mut out_pcm, channels, out_len_needed)
            .map_err(|e| SimpleDecoderError::ResampleError(e.to_string()))?;

        let mut indexing = Indexing {
            input_offset: 0,
            output_offset: 0,
            active_channels_mask: None,
            partial_len: None,
        };

        let mut input_frames_left = nbr;
        let mut input_frames_next = resampler.input_frames_next();

        while input_frames_left >= input_frames_next {
            let (frames_read, frames_writen) = resampler
                .process_into_buffer(&input_adapter, &mut output_adatper, Some(&indexing))
                .map_err(|e| SimpleDecoderError::ResampleError(e.to_string()))?;

            indexing.input_offset += frames_read;
            indexing.output_offset += frames_writen;
            input_frames_left -= frames_read;
            input_frames_next = resampler.input_frames_next();
        }

        if input_frames_left > 0 {
            indexing.partial_len = Some(input_frames_left);

            let (_, frames_written) = resampler
                .process_into_buffer(&input_adapter, &mut output_adatper, Some(&indexing))
                .map_err(|e| SimpleDecoderError::ResampleError(e.to_string()))?;

            indexing.output_offset += frames_written;
        }

        let actual = indexing.output_offset * channels;
        out_pcm.truncate(actual);

        Ok(Self {
            format: self.format,
            channels: self.channels,
            sample_rate: target_rate,
            pcm: out_pcm,
        })
    }

    pub fn rechannel(&self, target_channels: u16) -> Self {
        if self.channels == target_channels {
            return self.clone();
        }

        let mut new_pcm = Vec::new();

        match (self.channels, target_channels) {
            (2, 1) => {
                new_pcm.reserve(self.pcm.len() / 2);
                for frame in self.pcm.chunks_exact(2) {
                    let mono_sample = (frame[0] + frame[1]) * 0.5;
                    new_pcm.push(mono_sample);
                }
            }
            (1, 2) => {
                new_pcm.reserve(self.pcm.len() * 2);
                for &sample in &self.pcm {
                    new_pcm.push(sample);
                    new_pcm.push(sample);
                }
            }
            _ => {
                let src_ch = self.channels as usize;
                let dst_ch = target_channels as usize;
                let frames = self.pcm.len() / src_ch;
                new_pcm.resize(frames * dst_ch, 0.0);

                for f in 0..frames {
                    for c in 0..dst_ch {
                        let val = if c < src_ch {
                            self.pcm[f * src_ch + c]
                        } else {
                            self.pcm[f * src_ch]
                        };
                        new_pcm[f * dst_ch + c] = val;
                    }
                }
            }
        }

        Self {
            format: self.format,
            channels: target_channels,
            sample_rate: self.sample_rate,
            pcm: new_pcm,
        }
    }
}
