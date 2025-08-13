mod app;
use app::*;
use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const LANE_WIDTH: f32 = 50.0;

#[macroquad::main("Traffic Simulation")]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    let colors: Vec<Color> = vec![DARKBLUE, PINK, GOLD, LIGHTGRAY];

    let mut lights = vec![
        TrafficLight::new(
            Direction::North,
            WINDOW_WIDTH / 2.0 - 90.0,
            WINDOW_HEIGHT / 2.0 - 100.0,
            true,
        ),
        TrafficLight::new(
            Direction::South,
            WINDOW_WIDTH / 2.0 + LANE_WIDTH,
            WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
            false,
        ),
        TrafficLight::new(
            Direction::East,
            WINDOW_WIDTH / 2.0 + LANE_WIDTH,
            WINDOW_HEIGHT / 2.0 - 100.0,
            false,
        ),
        TrafficLight::new(
            Direction::West,
            WINDOW_WIDTH / 2.0 - 90.0,
            WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
            false,
        ),
    ];

    let mut last_change = get_time();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Up) {
            let start_x = WINDOW_WIDTH / 2.0;
            cars.push(Car::new(
                Direction::North,
                start_x,
                WINDOW_HEIGHT - LANE_WIDTH,
                0.0,
                -2.0,
                colors[rand::gen_range(0, colors.len())],
            ));
        }

        if is_key_pressed(KeyCode::Down) {
            let start_x = WINDOW_WIDTH / 2.0 - LANE_WIDTH;
            cars.push(Car::new(Direction::South, start_x, -LANE_WIDTH, 0.0, 2.0, colors[rand::gen_range(0, colors.len())]));
        }

        if is_key_pressed(KeyCode::Right) {
            let start_y = WINDOW_HEIGHT / 2.0;
            cars.push(Car::new(Direction::East, -LANE_WIDTH, start_y, 2.0, 0.0, colors[rand::gen_range(0, colors.len())]));
        }

        if is_key_pressed(KeyCode::Left) {
            let start_y = WINDOW_HEIGHT / 2.0 - LANE_WIDTH;
            cars.push(Car::new(Direction::West, WINDOW_WIDTH, start_y, -2.0, 0.0, colors[rand::gen_range(0, colors.len())]));
        }

        if is_key_pressed(KeyCode::R) {}

        cars.retain(|car| {
            car.x > -LANE_WIDTH * 2.0
                && car.x < WINDOW_WIDTH + LANE_WIDTH * 2.0
                && car.y > -LANE_WIDTH * 2.0
                && car.y < WINDOW_HEIGHT + LANE_WIDTH * 2.0
        });

        clear_background(Color::from_rgba(34, 139, 34, 255));

        draw_roads();
        draw_lane_markings();

        let now = get_time();
        if now - last_change >= 6.0 {
            for light in &mut lights {
                light.green = false;
            }
            let random_index = rand::gen_range(0, lights.len());
            lights[random_index].update();

            last_change = now;
        }

        for car in &mut cars {
            car.update();
            car.draw();
        }

        for light in &lights {
            light.draw();
        }

        next_frame().await;
    }
}

fn draw_roads() {
    draw_rectangle(
        0.0,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        WINDOW_WIDTH,
        LANE_WIDTH * 2.0,
        DARKGRAY,
    );

    draw_rectangle(
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        0.0,
        LANE_WIDTH * 2.0,
        WINDOW_HEIGHT,
        DARKGRAY,
    );
}

fn draw_lane_markings() {
    let mut start = vec2(WINDOW_WIDTH / 2.0, 0.0);
    let mut end = vec2(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT);
    draw_dashed_line(start, end, 15.0, 10.0, 1.0, YELLOW);
    start = vec2(0.0, WINDOW_HEIGHT / 2.0);
    end = vec2(WINDOW_WIDTH, WINDOW_HEIGHT / 2.0);
    draw_dashed_line(start, end, 15.0, 10.0, 1.0, YELLOW);
    draw_line(
        0.0,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        1.0,
        WHITE,
    );
    draw_line(
        0.0,
        WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
        1.0,
        WHITE,
    );
    draw_line(
        WINDOW_WIDTH / 2.0 + LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        WINDOW_WIDTH,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        1.0,
        WHITE,
    );
    draw_line(
        WINDOW_WIDTH / 2.0 + LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
        WINDOW_WIDTH,
        WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
        1.0,
        WHITE,
    );
    draw_line(
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        0.0,
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        1.0,
        WHITE,
    );
    draw_line(
        WINDOW_WIDTH / 2.0 + LANE_WIDTH,
        0.0,
        WINDOW_WIDTH / 2.0 + LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
        1.0,
        WHITE,
    );
    draw_line(
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
        WINDOW_WIDTH / 2.0 - LANE_WIDTH,
        WINDOW_HEIGHT,
        1.0,
        WHITE,
    );
    draw_line(
        WINDOW_WIDTH / 2.0 + LANE_WIDTH,
        WINDOW_HEIGHT / 2.0 + LANE_WIDTH,
        WINDOW_WIDTH / 2.0 + LANE_WIDTH,
        WINDOW_HEIGHT,
        1.0,
        WHITE,
    );
}

fn draw_dashed_line(
    start: Vec2,
    end: Vec2,
    dash_length: f32,
    gap_length: f32,
    thickness: f32,
    color: Color,
) {
    let direction = (end - start).normalize();
    let total_length = start.distance(end);

    let mut current_pos = start;
    let mut distance_travelled = 0.0;

    while distance_travelled < total_length {
        let segment_end =
            current_pos + direction * dash_length.min(total_length - distance_travelled);
        draw_line(
            current_pos.x,
            current_pos.y,
            segment_end.x,
            segment_end.y,
            thickness,
            color,
        );

        current_pos = segment_end + direction * gap_length;
        distance_travelled += dash_length + gap_length;
    }
}