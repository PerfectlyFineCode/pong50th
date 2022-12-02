use std::ffi::c_char;
use rand::Rng;
use raylib::ffi::MeasureTextEx;
use raylib::prelude::*;
use crate::ai::AI;
use crate::audiosystem::{play_sfx, SoundType};
use crate::debug::draw_line;
use crate::time;
use crate::vectorhelper::Vector2Ext;

pub struct Game {
    ball_position: Vector2,
    ball_velocity: Vector2,
    ball_speed: f32,
    ball_radius: f32,
    player_position: Vector2,
    player_size: Vector2,
    player_speed: f32,
    enemy_ai: AI,
    score: (i32, i32),
    time_since_last_score: f64,
    pub(crate) paused: bool,
    pub screen_width: i32,
    pub screen_height: i32,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            ball_position: /* center screen */ Vector2::new(rl.get_screen_width() as f32 / 2.0, rl.get_screen_height() as f32 / 2.0),
            ball_velocity: Vector2::UP + Vector2::LEFT * 0.5,
            ball_speed: 15.0,
            ball_radius: 10.0,
            player_position: Vector2::new(10.0, rl.get_screen_height() as f32 / 2.0 - 50.0),
            player_size: Vector2::new(10.0, 100.0),
            player_speed: 10.0,
            enemy_ai: AI::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
            score: (0, 0),
            screen_width: rl.get_screen_width(),
            screen_height: rl.get_screen_height(),
            time_since_last_score: 0.0,
            paused: true,
        }
    }

    pub fn set_screen_size(&mut self, width: i32, height: i32) {
        // branchless check for screen size change
        if self.screen_width != width || self.screen_height != height {
            self.screen_width = width;
            self.screen_height = height;
            self.reposition_entities();
        }
    }

    pub fn reposition_entities(&mut self) {
        self.player_position = Vector2::new(10.0,
                                            self.screen_height as f32 / 2.0 - 50.0);
        self.enemy_ai.update_screen_size(self.screen_width as f32,
                                         self.screen_height as f32);
        self.ball_position = Vector2::new(self.screen_width as f32 / 2.0,
                                          self.screen_height as f32 / 2.0);
    }

    fn countdown(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // count down from 3
        self.paused = rl.get_time() - self.time_since_last_score < 3.0;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.countdown(rl, thread);

        if self.paused {
            return;
        }

        self.enemy_ai.update();
        // check W and S keys
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.player_position.y -= self.player_speed;
            self.player_position.y = self.player_position.y.max(0.0);
        }
        else if rl.is_key_down(KeyboardKey::KEY_S) {
            self.player_position.y += self.player_speed;
            self.player_position.y = self.player_position.y.min(self.screen_height as f32 - self.player_size.y);
        }

        // update ball position and velocity
        self.ball_position += self.ball_velocity * (self.ball_speed / (self.screen_width as f32 / self.screen_height as f32));
        self.enemy_ai.update_ball(self.ball_position, self.ball_velocity, self.ball_speed, self.ball_radius);
        self.check_collision();
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        // draw center line
        self.draw_center_line(d, thread);
        // draw ball
        self.draw_ball(d, thread);
        // draw player
        self.draw_player(d, thread);
        // draw enemy
        self.draw_enemy(d, thread);
        // draw score
        self.draw_score(d, thread);
        // draw countdown
        self.draw_countdown(d, thread);
    }

    fn draw_countdown(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        // draw countdown format 3 2 1 GO
        let time = get_time();
        let time_since_last_score = time - self.time_since_last_score;
        let countdown = 3.0 - time_since_last_score;
        if countdown > 0.0 {
            // draw text with format 3 2 1
            let text = format!("{:.0}", countdown);
            let text_width = measure_text(&text, 100) as f32;
            let text_position = Vector2::new(self.screen_width as f32 / 2.0 - text_width / 2.0,
                                             self.screen_height as f32 / 2.0 - 50.0);
            d.draw_text(&text,
                        text_position.x as i32,
                        text_position.y as i32,
                        100,
                        Color::WHITE);
        }
    }

    fn draw_enemy(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {

        d.draw_rectangle_v(self.enemy_ai.position,
                           self.enemy_ai.size,
                           Color::WHITE);
    }

    fn draw_player(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        d.draw_rectangle_v(self.player_position,
                           self.player_size,
                           Color::WHITE);
    }

    fn draw_score(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {

        // draw score left of the center screen
        d.draw_text(&format!("{}", self.score.0),
                    (self.screen_width as f32 / 2.0 - 100.0) as i32,
                    50.0 as i32,
                    100,
                    Color::WHITE,
        );

        // draw score right of the center screen
        d.draw_text(&format!("{}", self.score.1),
                    (self.screen_width as f32 / 2.0 + 50.0) as i32,
                    50.0 as i32,
                    100,
                    Color::WHITE,
        );
    }

    fn draw_center_line(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        // draw dotted line in the middle of the screen to divide the field
        let mut i = 0;
        while i < self.screen_height {
            // ring width is 5 pixels
            d.draw_rectangle(self.screen_width as i32 / 2,
                             i,
                             5,
                             10,
                             Color::WHITE);
            i += 20;
        }
    }

    fn draw_ball(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        // draw ball
        d.draw_circle_v(self.ball_position, self.ball_radius, Color::WHITE);
    }

    fn check_collision(&mut self) {
        self.check_wall_collision();
        self.check_player_collision();
        self.check_enemy_collision();
    }

    fn check_enemy_collision(&mut self) {
        // check if ball is in player area only allow collision if ball is moving towards the player
        let collided = check_ball_collision(self.ball_position,
                                                      self.ball_radius,
                                                      self.ball_velocity,
                                                      self.enemy_ai.position,
                                                      self.enemy_ai.size,
                                                      false);

        if collided {
            // set ball position to the edge of the enemy
            self.ball_position.x = self.enemy_ai.position.x - self.enemy_ai.size.x - self.ball_radius / 2.0;

            // reflect ball velocity
            // self.ball_velocity.x *= -1.0;
            self.ball_velocity = reflect_ball(self.ball_velocity,
                                              self.ball_position,
                         self.enemy_ai.position - self.enemy_ai.size / 2.0);

            // play bounce sound
            play_sfx(SoundType::Bounce, 0.5, 1.0);
        }
    }

    fn check_player_collision(&mut self) {

        // check if ball is in player area only allow collision if ball is moving towards the player
        let collided = check_ball_collision(self.ball_position,
                                                              self.ball_radius,
                                                              self.ball_velocity,
                                                              self.player_position,
                                                              self.player_size,
                                                              true);

        if collided {
            // set ball position to the edge of the player
            self.ball_position.x = self.player_position.x + self.player_size.x + self.ball_radius;

            // reflect ball velocity
            // self.ball_velocity.x *= -1.0;
            self.ball_velocity = reflect_ball(self.ball_velocity,
                                              self.ball_position,
                         self.player_position - self.player_size / 2.0);

            // play bounce sound
            play_sfx(SoundType::Bounce, 0.5, 1.0);
        }
    }

    fn check_wall_collision(&mut self) {
        // check collision with walls
        // check for collision with left and right walls
        if self.ball_position.x < -self.ball_radius / 2.0 {
            // set ball position to center
            self.ball_position.x = self.screen_width as f32 / 2.0;
            self.ball_position.y = self.screen_height as f32 / 2.0;

            // randomize ball velocity via fastrand vector2
            self.ball_velocity = get_random_direction();

            // add score to right player
            self.score.1 += 1;
            self.time_since_last_score = get_time();
        }
        else if self.ball_position.x > self.screen_width as f32 + self.ball_radius / 2.0 {
            // set ball position to center
            self.ball_position.x = self.screen_width as f32 / 2.0;
            self.ball_position.y = self.screen_height as f32 / 2.0;

            // randomize ball velocity via fastrand vector2
            self.ball_velocity = get_random_direction();

            // add score to right player
            self.score.0 += 1;
            self.time_since_last_score = get_time();
        }

        // check for collision with top and bottom walls
        if self.ball_position.y < self.ball_radius {
            // set position to the top wall
            self.ball_position.y = self.ball_radius;
            self.ball_velocity.y *= -1.0;

            // play bounce sound
            play_sfx(SoundType::Bounce, 0.5, 1.0);
        }
        else if self.ball_position.y > self.screen_height as f32 - self.ball_radius {
            // set position to the bottom wall
            self.ball_position.y = self.screen_height as f32 - self.ball_radius;
            self.ball_velocity.y *= -1.0;

            // play bounce sound
            play_sfx(SoundType::Bounce, 0.5, 1.0);
        }
    }
}

fn check_ball_collision(ball_position: Vector2,
                        ball_radius: f32,
                        ball_velocity: Vector2,
                        player_position: Vector2,
                        player_size: Vector2,
                        is_player: bool) -> bool {
    if ball_position.x - ball_radius < player_position.x + player_size.x &&
        ball_position.x + ball_radius > player_position.x &&
        ball_position.y - ball_radius < player_position.y + player_size.y &&
        ball_position.y + ball_radius > player_position.y
        /* ball is in direction */ &&
        (is_player && ball_velocity.x < 0.0 || !is_player && ball_velocity.x > 0.0) {
        return true;
    }
    return false;
}

fn reflect_ball(v_in: Vector2, ball_position: Vector2, paddle_center: Vector2) -> Vector2 {
    let mut v_out = v_in;
    v_out.x *= -1.0;
    v_out
}

fn get_random_direction() -> Vector2 {
    let mut rng = rand::thread_rng();
    // generate random direction within a 45 degree angle
    let angle: f32 = rng.gen_range(0.0..45.0);
    let angle: f32 = angle.to_radians();
    let x = angle.cos();
    let y = angle.sin();
    let mut direction = Vector2::new(x, y);
    // randomize direction
    if rng.gen_range(0..2) == 0 {
        direction.x *= -1.0;
    }
    if rng.gen_range(0..2) == 0 {
        direction.y *= -1.0;
    }
    return direction;
}

fn get_time() -> f64 {
    unsafe {
        time::TIME
    }
}

