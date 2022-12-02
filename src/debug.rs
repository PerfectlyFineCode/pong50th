use raylib::prelude::*;
use crate::time;

pub static mut DRAW_LIST: Vec<DrawHandle> = Vec::new();

pub struct DrawHandle {
    pub draw_command: DrawCommand,
    pub start_time: f64,
    pub duration: f32,
}

pub enum DrawCommand {
    Line(Vector2, Vector2, Color),
    Circle(Vector2, f32, Color),
    Rectangle(Vector2, Vector2, Color),
}

// draw_line
pub fn draw_line(start_pos: Vector2, end_pos: Vector2, color: Color, duration: f32) {
    unsafe {
        DRAW_LIST.push(DrawHandle {
            draw_command: DrawCommand::Line(start_pos, end_pos, color),
            start_time: time::TIME,
            duration,
        });
    }
}

// draw_circle
pub fn draw_circle(position: Vector2, radius: f32, color: Color, duration: f32) {
    unsafe {
        DRAW_LIST.push(DrawHandle {
            draw_command: DrawCommand::Circle(position, radius, color),
            start_time: time::TIME,
            duration,
        });
    }
}

// draw_rectangle
pub fn draw_rectangle(position: Vector2, size: Vector2, color: Color, duration: f32) {
    unsafe {
        DRAW_LIST.push(DrawHandle {
            draw_command: DrawCommand::Rectangle(position, size, color),
            start_time: time::TIME,
            duration,
        });
    }
}