use macroquad::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Car {
    pub color: Color,
    pub direction: String,
    pub width: i32,
    pub height: i32,
    pub cord: (f32, f32),
    pub stoped: bool,
    pub road: String,
}


fn get_color(nb: u32) -> Color {
    match nb {
        1 => GREEN,
        2 => BLUE,
        3 => RED,
        _ => YELLOW,
    }
}
fn get_road(nb: u32) -> String {
    match nb {
        1 => "up".to_string(),
        2 => "down".to_string(),
        3 => "left".to_string(),
        _ => "right".to_string(),
    }
}

impl Car {
    pub fn new(color: u32, dir: String, w: i32, h: i32, cord: (f32, f32), stoped: bool,road:u32) -> Self {
        Self {
            color: get_color(color),
            direction: dir,
            width: w,
            height: h,
            cord,
            stoped,
            road:get_road(road),
        }
    }
}
