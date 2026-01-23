use macroquad::prelude::*;

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

pub fn draw_dashed_middle_lines(gap: f32) {
    let w = screen_width();
    let h = screen_height();
    let cx = w / 2.0;
    let cy = h / 2.0;

    let dash = 18.0;
    let space = 12.0;
    let thickness = 3.0;

    draw_dashed_line(
        vec2(cx, 0.0),
        vec2(cx, cy - gap),
        dash,
        space,
        thickness,
        YELLOW,
    );
    draw_dashed_line(
        vec2(cx, cy + gap),
        vec2(cx, h),
        dash,
        space,
        thickness,
        YELLOW,
    );

    draw_dashed_line(
        vec2(0.0, cy),
        vec2(cx - gap, cy),
        dash,
        space,
        thickness,
        YELLOW,
    );
    draw_dashed_line(
        vec2(cx + gap, cy),
        vec2(w, cy),
        dash,
        space,
        thickness,
        YELLOW,
    );
}

pub fn draw_dashed_line(
    start: Vec2,
    end: Vec2,
    dash_len: f32,
    gap_len: f32,
    thickness: f32,
    color: Color,
) {
    let dir = end - start;
    let len = dir.length();
    if len <= 0.0 {
        return;
    }

    let step_dir = dir / len;
    let mut dist = 0.0;

    while dist < len {
        let a = start + step_dir * dist;
        let b = start + step_dir * (dist + dash_len).min(len);

        draw_line(a.x, a.y, b.x, b.y, thickness, color);

        dist += dash_len + gap_len;
    }
}
