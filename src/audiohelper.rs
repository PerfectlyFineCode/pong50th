use raylib::ffi::{InitAudioDevice, LoadSound, LoadSoundFromWave, LoadWaveFromMemory, PlaySound, PlaySoundMulti, SetAudioStreamPitch, SetSoundVolume};
use raylib::prelude::*;

#[macro_export]
macro_rules! load_from_memory {
    ($path:expr) => {
        {
            let sound_data = include_bytes!($path);
            let wave = LoadWaveFromMemory(const_c!(".wav"),
                                          sound_data.as_ptr() as *const _,
                                          sound_data.len() as i32);
            LoadSoundFromWave(wave)
        }
    };
}