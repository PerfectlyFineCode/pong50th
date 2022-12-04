use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::{DrawTexture, free, LoadImageFromMemory, LoadTextureFromImage, Texture, UnloadImage, UnloadTexture};
use raylib::prelude::*;
use crate::const_c;
use crate::game::Game;
use crate::time::{TIME};

const CREDITS_DURATION: f32 = 6.0;

pub(crate) struct Credits {
    pub title: String,
    pub author: String,
    pub acknowledgements: Vec<String>,
    pub version: String,
    pub description: String,
    pub website: String,
    pub license: String,
    logo_texture: Texture,
}

impl Credits {
    pub fn new() -> Credits {
        Credits {
            title: String::from("Pong50th"),
            author: String::from("PerfectlyFineCode"),
            acknowledgements: vec![
                String::from("[raylib-rs] Paul Clement (github.com/deltaphc)")
            ],
            version: String::from("0.1.0"),
            description: String::from("A pong game celebrating 50th anniversary, made with raylib-rs"),
            website: String::from("github.com/perfectlyfinecode/pong50th"),
            license: String::from("MIT"),
            logo_texture: unsafe {
                let image_data = include_bytes!("..\\res\\images\\logo.png");
                let img = LoadImageFromMemory(const_c!(".png"),
                                              image_data.as_ptr() as *const _,
                                              image_data.len() as i32);

                let tex = LoadTextureFromImage(img);
                UnloadImage(img);
                tex
            },
        }
    }

    pub fn update(&mut self, game: &mut Game) {
        let time = unsafe { TIME };
        if time > CREDITS_DURATION as f64 {
            game.game_state = crate::game::GameState::Playing;
            game.time_since_last_score = time;
        }
    }

    pub fn draw_credits(&mut self, d: &mut RaylibDrawHandle) {
        let mut y = d.get_screen_height() as f32 / 2.0 - 50.0 - (self.acknowledgements.len() as f32 * 30.0) - 30.0*3.0 - 150.0;
        // center text x
        let x = d.get_screen_width() as f32 / 2.0;

        let title = format!("Title: {}", &self.title);
        let author = format!("By {}", &self.author);
        let version = format!("Version: {}", &self.version);
        let description = format!("Description: {}", &self.description);
        let website = format!("Website: {}", &self.website);
        let license = format!("License: {}", &self.license);

        let title_width = measure_text(&title, 48);
        let author_width = measure_text(&author, 48);
        let version_width = measure_text(&version, 20);
        let description_width = measure_text(&description, 20);
        let website_width = measure_text(&website, 20);
        let license_width = measure_text(&license, 20);

        let title_x = (x - title_width as f32 / 2.0) as i32;
        let author_x = (x - author_width as f32 / 2.0) as i32;
        let version_x = (x - version_width as f32 / 2.0) as i32;
        let description_x = (x - description_width as f32 / 2.0) as i32;
        let website_x = (x - website_width as f32 / 2.0) as i32;
        let license_x = (x - license_width as f32 / 2.0) as i32;

        // fade in and fade out credits
        let time = unsafe { TIME };
        let mut alpha = 1.0;
        if time < CREDITS_DURATION as f64 / 2.0 {
            alpha = time as f32;
        }
        else if time > CREDITS_DURATION as f64 / 4.0 {
            alpha = (CREDITS_DURATION as f64 - time) as f32;
        }

        let fade_out_color = Color::WHITE.fade(alpha);

        unsafe {
            let img = self.logo_texture;
            DrawTexture(img,
                        x as i32 - (img.width as f32 / 2.0) as i32,
                        y as i32,
                        ffi::Color {
                            r: fade_out_color.r,
                            g: fade_out_color.g,
                            b: fade_out_color.b,
                            a: fade_out_color.a,
                        });
            y += img.height as f32 + 50.0 as f32;
        }


        // draw title
        d.draw_text(title.as_str(), title_x, y as i32, 40, fade_out_color);
        y += 50.0;

        // draw author
        d.draw_text(author.as_str(), author_x, y as i32, 40, fade_out_color);
        y += 50.0;
        // draw acknowledgements
        for author in &self.acknowledgements {
            let author = format!("{}", author);
            let author_width = measure_text(&author, 20);
            let author_x = (x - author_width as f32 / 2.0) as i32;

            d.draw_text(author.as_str(), author_x, y as i32, 20, fade_out_color);
            y += 30.0;
        }
        // draw version
        d.draw_text(version.as_str(),  version_x, y as i32, 20, fade_out_color);
        y += 30.0;
        // draw description
        d.draw_text(description.as_str(), description_x, y as i32, 20, fade_out_color);
        y += 30.0;
        // draw website
        d.draw_text(website.as_str(), website_x, y as i32, 20, fade_out_color);
        y += 30.0;
        // draw license
        d.draw_text(license.as_str(), license_x, y as i32, 20, fade_out_color);
    }
}