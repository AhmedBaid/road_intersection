use macroquad::prelude::*;
#[derive(Clone)]
pub struct TrafficLight {
    pub state: String,
    pub timer: f32,
    count: i32,
}
const LIGHTS: [&str; 4] = ["down", "left", "up", "right"];
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            state: "down".to_string(),
            timer: 4.0,
            count: 0,
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            if self.count == 3 {
                self.count = 0;
            } else {
                self.count += 1;
            }
            self.state = self.get_state();
            self.timer = 4.0;
        }
    }
    pub fn get_state(&self) -> String {
        LIGHTS[self.count as usize].to_string()
    }

    pub fn draw_lights(&self) {
        let w = screen_width();
        let h = screen_height();
        let cx = w / 2.0;
        let cy = h / 2.0;
        let gap = 60.0;

        let r = 8.0;
        let top_left = if self.state == "down" { GREEN } else { RED };
        let top_right = if self.state == "left" { GREEN } else { RED };
        let bottom_left = if self.state == "right" { GREEN } else { RED };
        let bottom_right = if self.state == "up" { GREEN } else { RED };
        draw_circle(cx - gap - 15.0, cy - gap - 15.0, r, top_left);

        draw_circle(cx + gap + 15.0, cy - gap - 15.0, r, top_right);
        
        draw_circle(cx - gap - 15.0, cy + gap + 15.0, r, bottom_left);

        draw_circle(cx + gap + 15.0, cy + gap + 15.0, r, bottom_right);

    }
}
