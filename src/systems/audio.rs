use rodio::source::Source;
use rodio::{Decoder, MixerDeviceSink, Player};
use slab::Slab;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::Result;

pub struct Audio {
    stream: MixerDeviceSink,
    music: Player,
    effects: Slab<Player>,
}

impl Audio {
    pub fn new(volume: f32) -> Result<Self> {
        let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        let music = rodio::Player::connect_new(stream_handle.mixer());
        music.set_volume(volume);
        Ok(Self {
            stream: stream_handle,
            music,
            effects: Slab::new(),
        })
    }

    pub fn set_music(&mut self, source: impl AsRef<Path>) -> Result<()> {
        let file = BufReader::new(File::open(source)?);
        let source = Decoder::new(file)?;

        self.music.append(
            source
                .fade_in(Duration::from_secs(1))
                .repeat_infinite()
                .skippable(),
        );

        if self.music.len() > 1 {
            self.music.skip_one()
        }

        self.music.play();
        Ok(())
    }

    pub fn stop_music(&mut self) {
        self.music.clear();
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music.set_volume(volume);
    }

    pub fn play_effect(
        &mut self,
        source: impl AsRef<Path>,
        volume: f32,
    ) -> Result<()> {
        let sink = rodio::Player::connect_new(self.stream.mixer());
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
