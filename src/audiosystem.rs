use lazy_static::lazy_static;
use raylib::ffi::{Sound, LoadSound, PlaySound, PlaySoundMulti, SetSoundPitch, SetSoundVolume, SetAudioStreamPitch, SetAudioStreamVolume, PlayAudioStream};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::const_c;

pub struct SoundHandle {
    pub sound_type: SoundType,
    pub volume: f32,
    pub pitch: f32,
}

#[derive(Eq, Hash, PartialEq)]
pub enum SoundType {
    Bounce,
    Score,
    Wall,
}

pub(crate) static mut SOUND_STACK: Vec<SoundHandle> = Vec::new();

pub struct SoundManager {
    sounds: HashMap<SoundType, Sound>,
}

impl SoundManager {
    pub fn new(map: Option<HashMap<SoundType, Sound>>) -> SoundManager {
        SoundManager {
            sounds: map.unwrap_or_else(|| { HashMap::new() }),
        }
    }

    pub fn load(&mut self, sound_type: SoundType, path: &str) {
        let sound = unsafe { LoadSound(const_c!(path)) };
        self.sounds.insert(sound_type, sound);
    }

    pub fn play(&self, sound_type: &SoundType, volume: f32, pitch: f32) {
        let sound = self.sounds.get(sound_type).unwrap();
        unsafe {
            let c_sound = sound.clone();
            SetAudioStreamPitch(c_sound.stream, pitch);
            SetAudioStreamVolume(c_sound.stream, volume);
            PlayAudioStream(c_sound.stream);
        };
    }
}

pub fn play_sfx(sound_type: SoundType, volume: f32, pitch: f32) {
    unsafe {
        SOUND_STACK.push(SoundHandle {
            sound_type,
            volume,
            pitch,
        });
    }
}