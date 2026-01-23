use macroquad::prelude::*;
mod dashed;
mod draw_road;
mod tools;
use draw_road::*;
use tools::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
        window_width: 1000,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    loop {
        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_road();
        if is_key_pressed(KeyCode::Up) {
            let cord = (screen_width() / 2.0 + 15.0, screen_height() - 35.0);
            let car = Car::new(
                rand::gen_range(1, 5),
                "up".to_string(),
                30,
                30,
                cord,
                true,
                rand::gen_range(1, 5),
            );
            cars.push(car);
        }
        if is_key_pressed(KeyCode::Right) {
            let cord = (10.0, screen_height() / 2.0 + 15.0);
            let car = Car::new(
                rand::gen_range(1, 5),
                "right".to_string(),
                30,
                30,
                cord,
                true,
                rand::gen_range(1, 4),
            );
            cars.push(car);
        }
        if is_key_pressed(KeyCode::Down) {
            let cord = (screen_width() / 2.0 - 45.0, 10.0);
            let car = Car::new(
                rand::gen_range(1, 5),
                "down".to_string(),
                30,
                30,
                cord,
                true,
                rand::gen_range(1, 4),
            );
            cars.push(car);
        }
        if is_key_pressed(KeyCode::Left) {
            let cord = (screen_width() - 35.0, screen_height() / 2.0 - 45.0);
            let car = Car::new(
                rand::gen_range(1, 4),
                "left".to_string(),
                30,
                30,
                cord,
                true,
                rand::gen_range(1, 4),
            );
            cars.push(car);
        }
        for car in &cars {
            draw_rectangle(
                car.cord.0,
                car.cord.1,
                car.width as f32,
                car.height as f32,
                car.color,
            );
        }
        next_frame().await
    }
}
