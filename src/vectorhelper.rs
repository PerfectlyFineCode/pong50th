use raylib::prelude::Vector2;

// extend vector2 struct
pub trait Vector2Ext {
    // reflect vector2
    fn reflect(&self, normal: &Vector2) -> Vector2;

    // UP
    const UP: Vector2;
    // DOWN
    const DOWN: Vector2;
    // LEFT
    const LEFT: Vector2;
    // RIGHT
    const RIGHT: Vector2;

    const ZERO: Vector2;

    const ONE: Vector2;
}

impl Vector2Ext for Vector2 {

    fn reflect(&self, normal: &Vector2) -> Vector2 {
        let dot = self.dot(*normal);
        let mut result = normal.clone();
        result *= dot * 2.0;
        // subtract result from self creating a new vector vector2::new(self.x - result.x, self.y - result.y)
        result -= *self;
        result
    }

    // UP
    const UP: Vector2 = Vector2::new(0.0, -1.0);
    // DOWN
    const DOWN: Vector2 = Vector2::new(0.0, 1.0);
    // LEFT
    const LEFT: Vector2 = Vector2::new(-1.0, 0.0);
    // RIGHT
    const RIGHT: Vector2 = Vector2::new(1.0, 0.0);

    const ZERO: Vector2 = Vector2::new(0.0, 0.0);

    const ONE: Vector2 = Vector2::new(1.0, 1.0);
}