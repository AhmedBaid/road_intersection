use macroquad::prelude::*;
mod cars;
mod dashed;
mod draw_road;
mod lights;
use cars::*;
use draw_road::*;
use lights::*;

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

// Helper to check if it's safe to spawn a new car
fn can_spawn(cars: &Vec<Car>, direction: &str, spawn_cord: (f32, f32)) -> bool {
    let safe_dist = 60.0;
    for car in cars {
        if car.direction == direction {
            // Simple distance check
            let dist = (
                (car.cord.0 - spawn_cord.0).powi(2) + (car.cord.1 - spawn_cord.1).powi(2)
            ).sqrt();
            if dist < safe_dist {
                return false;
            }
        }
    }
    true
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    let mut traffic_light = TrafficLight::new();

    // Safety gap for following cars
    let safety_gap = 50.0;

    loop {
        let dt = get_frame_time();

        traffic_light.update(dt);
        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_road();
        traffic_light.draw_lights();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            cars.clear();
        }

        // --- Spawning Logic with Safety Checks ---

        if is_key_pressed(KeyCode::Up) {
            let cord = (screen_width() / 2.0 + 15.0, screen_height() - 35.0);
            if can_spawn(&cars, "up", cord) {
                cars.push(Car::new("up".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let cord = (10.0, screen_height() / 2.0 + 15.0);
            if can_spawn(&cars, "right", cord) {
                cars.push(Car::new("right".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let cord = (screen_width() / 2.0 - 45.0, 10.0);
            if can_spawn(&cars, "down", cord) {
                cars.push(Car::new("down".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let cord = (screen_width() - 35.0, screen_height() / 2.0 - 45.0);
            if can_spawn(&cars, "left", cord) {
                cars.push(Car::new("left".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        // --- Update Logic with Collision Checks ---

        // We use indices to access cars to avoid simultaneous borrow issues
        for i in 0..cars.len() {
            let mut blocked = false;
            let my_cord = cars[i].cord;
            let my_dir = cars[i].direction.clone();

            // Check against every other car
            for j in 0..cars.len() {
                if i == j {
                    continue;
                }

                let other = &cars[j];

                // Only check collision with cars from the same origin direction
                if my_dir == other.direction {
                    let other_cord = other.cord;

                    // Calculate distance
                    let dist = (
                        (my_cord.0 - other_cord.0).powi(2) + (my_cord.1 - other_cord.1).powi(2)
                    ).sqrt();

                    if dist < safety_gap {
                        // Check if the 'other' car is logically in front
                        let is_ahead = match my_dir.as_str() {
                            "up" => other_cord.1 < my_cord.1, // Moving up (negative Y)
                            "down" => other_cord.1 > my_cord.1, // Moving down (positive Y)
                            "left" => other_cord.0 < my_cord.0, // Moving left (negative X)
                            "right" => other_cord.0 > my_cord.0, // Moving right (positive X)
                            _ => false,
                        };

                        if is_ahead {
                            blocked = true;
                            break;
                        }
                    }
                }
            }

            cars[i].update(dt, traffic_light.get_state(), blocked);
        }

        // cleanup cars that left the screen
        cars = cars
            .iter()
            .filter(|car| {
                let (x, y) = car.cord;
                x > -30.0 && x < screen_width() + 30.0 && y > -30.0 && y < screen_height() + 30.0
            })
            .cloned()
            .collect();

        for car in &cars {
            draw_rectangle(car.cord.0, car.cord.1, car.width as f32, car.height as f32, car.color);
        }

        next_frame().await;
    }
}
