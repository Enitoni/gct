//! This file adds compatibility between Songbird and the decoupled audio engine

use std::io::Seek;

use crate::audio::{AudioBufferConsumer, AudioSystem, CHANNEL_COUNT, PCM_MIME, SAMPLE_RATE};
use songbird::input::{Input, LiveInput, RawAdapter};
use symphonia::core::{io::MediaSource, probe::Hint};

impl MediaSource for AudioBufferConsumer {
    fn byte_len(&self) -> Option<u64> {
        None
    }

    fn is_seekable(&self) -> bool {
        false
    }
}

impl AudioSystem {
    fn source(&self) -> Box<dyn MediaSource> {
        let adapter = RawAdapter::new(self.stream(), SAMPLE_RATE as u32, CHANNEL_COUNT as u32);

        Box::new(adapter)
    }

    pub(super) fn create_input(&self) -> Input {
        let mut hint = Hint::new();
        hint.mime_type(PCM_MIME);

        let stream = songbird::input::AudioStream {
            input: self.source(),
            hint: Some(hint),
        };

        let input = LiveInput::Raw(stream);

        Input::Live(input, None)
    }
}

impl Seek for AudioBufferConsumer {
    fn seek(&mut self, _seek: std::io::SeekFrom) -> std::io::Result<u64> {
        // This is a no op
        Ok(0)
    }
}
