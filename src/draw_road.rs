use macroquad::prelude::*;
use crate::dashed::*;
pub fn draw_road() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let tickness = 3.0;
    let gap = 60.0;
    let color1 = GRAY;
    draw_line(
        (screen_width / 2.0) - gap,
        (screen_height / 2.0) - gap,
        (screen_width / 2.0) - gap,
        0.0,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) + gap,
        (screen_height / 2.0) - gap,
        (screen_width / 2.0) + gap,
        0.0,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) + gap,
        (screen_height / 2.0) - gap,
        screen_width,
        (screen_height / 2.0) - gap,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) + gap,
        (screen_height / 2.0) + gap,
        screen_width,
        (screen_height / 2.0) + gap,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) + gap,
        (screen_height / 2.0) + gap,
        (screen_width / 2.0) + gap,
        screen_height,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) - gap,
        (screen_height / 2.0) + gap,
        (screen_width / 2.0) - gap,
        screen_height,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) - gap,
        (screen_height / 2.0) + gap,
        0.0,
        (screen_height / 2.0) + gap,
        tickness,
        color1,
    );
    draw_line(
        (screen_width / 2.0) - gap,
        (screen_height / 2.0) - gap,
        0.0,
        (screen_height / 2.0) - gap,
        tickness,
        color1,
    );
    draw_dashed_middle_lines(20.0);
}
