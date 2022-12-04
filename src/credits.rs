use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use crate::time::DELTA_TIME;

const CREDITS_SPEED: f32 = 100.0;
const CREDITS_DURATION: f32 = 10.0;

pub(crate) struct Credits {
    pub title: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: String,
    pub website: String,
    pub license: String,
    offset: f32,
}

impl Credits {
    pub fn new() -> Credits {
        Credits {
            title: String::from("Pong50th"),
            authors: vec![String::from("PerfectlyFineCode")],
            version: String::from("0.1.0"),
            description: String::from("A pong game celebrating 50th anniversary, made with raylib-rs"),
            website: String::from("github.com/perfectlyfinecode/pong50th"),
            license: String::from("MIT"),
            offset: 0.0,
        }
    }

    pub fn update(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        // scroll credits for duration of CREDITS_DURATION
        self.offset += CREDITS_SPEED * delta_time;
    }

    pub fn draw_credits(&mut self, d: &mut RaylibDrawHandle) {
        let mut y = self.offset;

        // draw title
        d.draw_text(&self.title, 10, y as i32, 40, Color::WHITE);
        y += 50.0;
        // draw authors
        for author in &self.authors {
            d.draw_text(&author, 10, y as i32, 20, Color::WHITE);
            y += 30.0;
        }
        // draw version
        d.draw_text(&self.version, 10, y as i32, 20, Color::WHITE);
        y += 30.0;
        // draw description
        d.draw_text(&self.description, 10, y as i32, 20, Color::WHITE);
        y += 30.0;
        // draw website
        d.draw_text(&self.website, 10, y as i32, 20, Color::WHITE);
        y += 30.0;
        // draw license
        d.draw_text(&self.license, 10, y as i32, 20, Color::WHITE);

    }
}