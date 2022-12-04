#![windows_subsystem = "windows"]

mod game;
mod vectorhelper;
mod ai;
mod time;
mod debug;
mod stringhelper;
mod audiosystem;
mod audiohelper;
mod floathelper;
mod credits;
mod gamestate;

use std::collections::HashMap;
use raylib::ffi::{InitAudioDevice, LoadSound, LoadSoundFromWave, LoadWaveFromMemory, PlaySound, PlaySoundMulti, SetAudioStreamPitch, SetSoundVolume};
use raylib::prelude::*;
use crate::debug::DRAW_LIST;

fn main() {
    let (mut rl, thread) = init()
        .vsync()
        .size(800, 450)
        .title("raylib [core] example - basic window")
        .resizable()
        .build();

    // initialize raylib audio
    unsafe { InitAudioDevice(); };

    // rl.set_target_fps(120);
    let mut state = rl.get_window_state();
    state = state.set_vsync_hint(true)
        .set_window_undecorated(true)
        .set_window_maximized(true);
    rl.set_window_state(state);

    // load sounds
    let mut sound_map = HashMap::new();
    sound_map.insert(audiosystem::SoundType::Bounce, unsafe {
        load_from_memory!("..\\res\\sounds\\paddle_hit.wav")
    });
    sound_map.insert(audiosystem::SoundType::PlayerScored, unsafe {
        load_from_memory!("..\\res\\sounds\\win.wav")
    });
    sound_map.insert(audiosystem::SoundType::EnemyScored, unsafe {
        load_from_memory!("..\\res\\sounds\\lose.wav")
    });

    // initialize audiomanager with sounds from sound_map
    let mut audio_manager = audiosystem::SoundManager::new(Some(sound_map));

    let mut game = game::Game::new(&mut rl);
    game.time_since_last_score = 5.0;
    let mut credits = credits::Credits::new();

    while !rl.window_should_close() {
        let time = &rl.get_time();

        // update time
        unsafe {
            time::TIME = time.clone();
            time::DELTA_TIME = rl.get_frame_time();
        }

        // game.update(&mut rl, &thread);
        if game.game_state == game::GameState::Playing {
            game.update(&mut rl);
        } else if game.game_state == game::GameState::Credits {
            credits.update();
        }
        // game.update(&mut rl);

        let mut d = rl.begin_drawing(&thread);

        credits.draw_credits(&mut d);

        let bounds = (d.get_screen_width(), d.get_screen_height());
        game.set_screen_size(bounds.0, bounds.1);

        d.clear_background(Color::BLACK);
        if game.game_state == game::GameState::Playing {
            game.draw(&mut d);
        }
        else if game.game_state == game::GameState::Credits {
            credits.draw_credits(&mut d);
        }
        // game.draw(&mut d);

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