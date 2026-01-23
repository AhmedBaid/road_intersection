use macroquad::prelude::*;
#[derive(Clone)]
pub struct Car {
    pub direction: String,
    pub width: i32,
    pub height: i32,
    pub cord: (f32, f32),
    pub color: Color,
    pub speed: f32,
}

impl Car {
    pub fn new(
        direction: String,
        width: i32,
        height: i32,
        cord: (f32, f32),
        color: i32,
    ) -> Self {
        let color = match color {
            1 => YELLOW,
            2 => BLUE,
            3 => RED,
            _ => GREEN,
        };

        Self {
            direction,
            width,
            height,
            cord,
            color,
            speed: 160.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let (mut x, mut y) = self.cord;

        match self.direction.as_str() {
            "up" => y -= self.speed * dt,
            "down" => y += self.speed * dt,
            "left" => x -= self.speed * dt,
            "right" => x += self.speed * dt,
            _ => {}
        }

        self.cord = (x, y);
    }
}
