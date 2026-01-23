use macroquad::prelude::*;
mod draw_road;
use draw_road::*;
#[macroquad::main("road_intersection")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_road();
        
        next_frame().await
    }
}
