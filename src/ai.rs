use raylib::prelude::*;

pub struct AI {
    ball_position: Vector2,
    ball_velocity: Vector2,
    ball_speed: f32,
    ball_radius: f32,
    pub position: Vector2,
    pub size: Vector2,
    pub speed: f32,
    screen_width: f32,
    screen_height: f32,
}

impl AI {
    pub fn new(screen_width: f32, screen_height: f32) -> AI {
        AI {
            ball_position: Default::default(),
            ball_velocity: Default::default(),
            ball_speed: Default::default(),
            ball_radius: Default::default(),
            position: Vector2::new(screen_width - 20.0, screen_height / 2.0 - 50.0),
            size: Vector2::new(10.0, 100.0),
            speed: 6.0,
            screen_width,
            screen_height,
        }
    }

    pub fn update_screen_size(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;

        // update position
        self.position.x = screen_width - 20.0;
        self.position.y = screen_height / 2.0 - 50.0;
    }

    pub fn update_ball(&mut self, ball_position: Vector2, ball_velocity: Vector2, ball_speed: f32, ball_radius: f32) {
        self.ball_position = ball_position;
        self.ball_velocity = ball_velocity;
        self.ball_speed = ball_speed;
        self.ball_radius = ball_radius;
    }

    pub fn update(&mut self) {
        // move Y towards ball
        if self.position.y + self.size.y / 2.0 < self.ball_position.y {
            self.position.y += self.speed;
        }
        else if self.position.y + self.size.y / 2.0 > self.ball_position.y {
            self.position.y -= self.speed;
        }

        self.clamp_to_screen();
    }

    fn clamp_to_screen(&mut self) {
        // clamp Y to screen
        self.position.y = self.position.y.clamp(0.0,
                                                self.screen_height - self.size.y);
        self.position.x = self.screen_width - 20.0;
    }

}