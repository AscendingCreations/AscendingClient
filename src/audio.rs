use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use slab::Slab;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::Result;

pub struct Audio {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    music: Sink,
    effects: Slab<Sink>,
}

impl Audio {
    pub fn new(volume: f32) -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let music = Sink::try_new(&stream_handle)?;
        music.set_volume(volume);
        Ok(Self {
            stream_handle,
            _stream: stream,
            music,
            effects: Slab::new(),
        })
    }

    pub fn set_music(&mut self, source: impl AsRef<Path>) -> Result<()> {
        self.music.clear();

        let file = BufReader::new(File::open(source)?);
        let source = Decoder::new(file)?;

        self.music.append(source.repeat_infinite());
        self.music.play();
        Ok(())
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music.set_volume(volume);
    }

    pub fn play_effect(
        &mut self,
        source: impl AsRef<Path>,
        volume: f32,
    ) -> Result<()> {
        let sink = Sink::try_new(&self.stream_handle)?;
        let file = BufReader::new(File::open(source)?);
        let source = Decoder::new(file)?;
        sink.set_volume(volume);
        sink.append(source);
        sink.play();
        self.effects.insert(sink);
        Ok(())
    }

    pub fn set_effect_volume(&mut self, volume: f32) {
        for effect in &mut self.effects {
            effect.1.set_volume(volume);
        }
    }

    pub fn update_effects(&mut self) {
        let mut rem = Vec::new();

        for (id, effect) in &self.effects {
            if effect.len() == 0 {
                rem.push(id);
            }
        }

        for id in rem {
            self.effects.remove(id);
        }
    }
}
