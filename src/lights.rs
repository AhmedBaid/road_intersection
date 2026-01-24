use macroquad::prelude::*;
#[derive(Clone)]
pub struct TrafficLight {
    pub state: String,
    pub timer: f32,
    count: i32,
    base_duration: f32,
    current_duration: f32, 
}
const LIGHTS: [&str; 4] = ["down", "left", "up", "right"];
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            state: "down".to_string(),
            timer: 4.0,
            count: 0,
            base_duration: 4.0,    
            current_duration: 4.0,
        }
    }


    pub fn update_with_congestion(
        &mut self, 
        dt: f32, 
        up_count: usize, 
        down_count: usize, 
        left_count: usize, 
        right_count: usize,
        capacity: usize
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            // Calculate next state and duration based on congestion
            let (next_state, next_duration) = self.calculate_next_state(
                up_count, down_count, left_count, right_count, capacity
            );
            
            self.state = next_state;
            self.current_duration = next_duration;
            self.timer = next_duration;
        }
    }

fn calculate_next_state(
    &mut self,  
    up_count: usize,
    down_count: usize,
    left_count: usize,
    right_count: usize,
    capacity: usize,
) -> (String, f32) {
    // Create vec of (direction, count, index)
    let mut lanes = vec![
        ("up", up_count, 2),
        ("down", down_count, 0),
        ("left", left_count, 1),
        ("right", right_count, 3),
    ];

    // Find the most congested lane
    lanes.sort_by(|a, b| b.1.cmp(&a.1));
    
    let most_congested = lanes[0];
    let congestion_ratio = most_congested.1 as f32 / capacity as f32;

    // Priority logic:
    // 1. If a lane is at or above 80% capacity, give it priority
    // 2. Otherwise, cycle normally but adjust duration based on traffic
    
    if congestion_ratio >= 0.8 {
        // Emergency: Give priority to most congested lane
        let duration = (self.base_duration * 1.5).min(8.0); // Max 8 seconds
        
        // ← FIX: Update count to match the priority direction
        self.count = most_congested.2;
        
        (most_congested.0.to_string(), duration)
    } else {
        // Normal cycling - move to next direction
        if self.count == 3 {
            self.count = 0;
        } else {
            self.count += 1;
        }
        
        let next_direction = LIGHTS[self.count as usize].to_string();
        
        // Check traffic for this direction
        let next_count = match next_direction.as_str() {
            "up" => up_count,
            "down" => down_count,
            "left" => left_count,
            "right" => right_count,
            _ => 0,
        };
        
        let next_ratio = next_count as f32 / capacity as f32;
        
        // Adjust duration based on traffic
        let duration = if next_ratio > 0.5 {
            self.base_duration * 1.25 // 5 seconds if busy
        } else if next_count > 0 {
            self.base_duration // 4 seconds if some traffic
        } else {
            self.base_duration * 0.75 // 3 seconds if empty
        };
        
        (next_direction, duration)
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


    // Top-left margin
    let margin_x = 10.0;
    let margin_y = 10.0;

    // Timer at top-left
    draw_text(
        &format!("{:.1}s", self.timer),
        margin_x,
        margin_y + 24.0, // add text height so it’s visible
        24.0,
        WHITE,
    );

    // Current green direction below timer
    draw_text(
        &format!("Green: {}", self.state.to_uppercase()),
        margin_x,
        margin_y + 24.0 + 24.0 + 5.0, // 24 for timer + 24 for font size + 5 spacing
        20.0,
        GREEN,
    );

    }
}
