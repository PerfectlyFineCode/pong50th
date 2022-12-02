mod game;
mod vectorhelper;
mod ai;
mod time;
mod debug;
mod stringhelper;
mod audiosystem;
mod audiohelper;

use std::collections::HashMap;
use raylib::ffi::{InitAudioDevice, LoadSound, LoadSoundFromWave, LoadWaveFromMemory, PlaySound, PlaySoundMulti, SetAudioStreamPitch, SetSoundVolume};
use raylib::prelude::*;
use crate::debug::DRAW_LIST;

fn main() {
    let (mut rl, thread) = init()
        .size(800, 450)
        .title("raylib [core] example - basic window")
        .build();

    // initialize raylib audio
    unsafe { InitAudioDevice(); };

    // borderless window
    // rl.set_window_position(0, 0);
    // rl.set_window_size(1920, 1080);
    let state = rl.get_window_state();
    rl.set_target_fps(120);
    rl.set_window_monitor(0);

    // load sounds
    let mut sound_map = HashMap::new();
    sound_map.insert(audiosystem::SoundType::Bounce, unsafe {
        load_from_memory!("..\\res\\sounds\\paddle_hit.wav")
    });

    // initialize audiomanager
    let mut audio_manager = audiosystem::SoundManager::new(Some(sound_map));

    let mut game = game::Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let time = &rl.get_time();

        // update time
        unsafe {
            time::TIME = time.clone();
            time::DELTA_TIME = rl.get_frame_time();
        }

        // game.update(&mut rl, &thread);
        game.update(&mut rl, &thread);

        let mut d = rl.begin_drawing(&thread);

        let bounds = (d.get_screen_width(), d.get_screen_height());
        game.set_screen_size(bounds.0, bounds.1);

        d.clear_background(Color::BLACK);
        game.draw(&mut d, &thread);

        // play audio effects on stack
        unsafe {
            for sound_handle in audiosystem::SOUND_STACK.drain(..) {
                audio_manager.play(&sound_handle.sound_type, sound_handle.volume, sound_handle.pitch);
            }
        }

        if cfg!(debug_assertions) {
            draw_debug(&time, &mut d);
        }

        // draw FPS top left corner
        d.draw_text(&format!("FPS: {}", d.get_fps()),
                    10,
                    10,
                    20,
                    Color::GRAY);
    }
}

pub fn draw_debug(time: &f64, d: &mut RaylibDrawHandle) {
    // draw debug stack
    unsafe {
        // draw debug until duration is up
        for draw_command in DRAW_LIST.iter() {
            if draw_command.start_time + draw_command.duration as f64 > *time {
                match draw_command.draw_command {
                    debug::DrawCommand::Line(start_pos, end_pos, color) => {
                        d.draw_line(start_pos.x as i32,
                                    start_pos.y as i32,
                                    end_pos.x as i32,
                                    end_pos.y as i32,
                                    color);
                    }
                    debug::DrawCommand::Circle(position, radius, color) => {
                        d.draw_circle(position.x as i32,
                                      position.y as i32,
                                      radius,
                                      color);
                    }
                    debug::DrawCommand::Rectangle(position, size, color) => {
                        d.draw_rectangle(position.x as i32,
                                         position.y as i32,
                                         size.x as i32,
                                         size.y as i32,
                                         color);
                    }
                }
            }
        }
        DRAW_LIST.retain(|x| (x.start_time + x.duration as f64) > *time);
    }
}