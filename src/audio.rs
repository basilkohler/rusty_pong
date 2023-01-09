use rusty_audio::prelude::*;

pub struct AudioPlayer(Audio);

impl AudioPlayer {
    pub fn new() -> Self {
        let mut audio = Audio::new();

        for clip_name in [ "explode", "lose", "move", "pew", "startup", "win"] {
            audio.add(clip_name, format!("audio/{clip_name}.wav"));
        }

        AudioPlayer(audio)
    }

    pub fn play(&mut self, audio_clip: AudioClip) {
        let clip_name = match audio_clip {
            AudioClip::Startup => "startup",
            AudioClip::Win => "win",
            AudioClip::Lose => "lose",
            AudioClip::Move => "move",
            AudioClip::Pew => "pew",
            AudioClip::Explode => "explode",
        };
        self.0.play(clip_name);
    }

    pub fn wait(&mut self) {
        self.0.wait();
    }
}

pub enum AudioClip {
    Startup,
    Win,
    Lose,
    Move,
    Pew,
    Explode,
}