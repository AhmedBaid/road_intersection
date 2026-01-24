use macroquad::prelude::*;
mod dashed;
mod draw_road;
mod tools;
use draw_road::*;
use tools::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
        window_width: 900,
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
        let dt = get_frame_time();

        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_road();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            cars.clear();
        }

        if is_key_pressed(KeyCode::Up) {
            let cord = (screen_width() / 2.0 + 15.0, screen_height() - 35.0);
            cars.push(Car::new(
                "up".to_string(),
                30,
                30,
                cord,
                rand::gen_range(1, 4),
            ));
        }

        if is_key_pressed(KeyCode::Right) {
            let cord = (10.0, screen_height() / 2.0 + 15.0);
            cars.push(Car::new(
                "right".to_string(),
                30,
                30,
                cord,
                rand::gen_range(1, 4),
            ));
        }

        if is_key_pressed(KeyCode::Down) {
            let cord = (screen_width() / 2.0 - 45.0, 10.0);
            cars.push(Car::new(
                "down".to_string(),
                30,
                30,
                cord,
                rand::gen_range(1, 4),
            ));
        }

        if is_key_pressed(KeyCode::Left) {
            let cord = (screen_width() - 35.0, screen_height() / 2.0 - 45.0);
            cars.push(Car::new(
                "left".to_string(),
                30,
                30,
                cord,
                rand::gen_range(1, 4),
            ));
        }

        for car in cars.iter_mut() {
            car.update(dt);
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
