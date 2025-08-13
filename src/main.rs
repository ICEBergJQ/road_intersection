mod app;
use ::rand::Rng;
use ::rand::thread_rng;
use app::*;
use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const LANE_WIDTH: f32 = 50.0;

#[macroquad::main("Traffic Simulation")]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    let colors: Vec<(Col, Turn)> = vec![
        (Col::Darkblue, Turn::Left),
        (Col::Pink, Turn::Right),
        (Col::Gold, Turn::Front),
    ];

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
            let mut rng = thread_rng();
            let (col, turn) = colors[rng.gen_range(0..colors.len())];
            let color = Color::from(col);
            let start_x = WINDOW_WIDTH / 2.0;
            if can_spawn(&cars, start_x, 50.0, 0.0, -2.0) {
                cars.push(Car::new(
                    Direction::North,
                    start_x,
                    WINDOW_HEIGHT + LANE_WIDTH,
                    0.0,
                    -2.0,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let mut rng = thread_rng();
            let (col, turn) = colors[rng.gen_range(0..colors.len())];
            let color = Color::from(col);
            let start_x = WINDOW_WIDTH / 2.0 - LANE_WIDTH;
            if can_spawn(&cars, start_x, 50.0, 0.0, 2.0) {
                cars.push(Car::new(
                    Direction::South,
                    start_x,
                    -LANE_WIDTH,
                    0.0,
                    2.0,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let mut rng = thread_rng();
            let (col, turn) = colors[rng.gen_range(0..colors.len())];
            let color = Color::from(col);
            let start_y = WINDOW_HEIGHT / 2.0;
            if can_spawn(&cars, start_y, 50.0, 2.0, 0.0) {
                cars.push(Car::new(
                    Direction::East,
                    -LANE_WIDTH,
                    start_y,
                    2.0,
                    0.0,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let mut rng = thread_rng();
            let (col, turn) = colors[rng.gen_range(0..colors.len())];
            let color = Color::from(col);
            let start_y = WINDOW_HEIGHT / 2.0 - LANE_WIDTH;
            if can_spawn(&cars, start_y, 50.0, -2.0, 0.0) {
                cars.push(Car::new(
                    Direction::West,
                    WINDOW_WIDTH + LANE_WIDTH,
                    start_y,
                    -2.0,
                    0.0,
                    color,
                    turn,
                ));
            }
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
            let mut car_can_move = true;

            for light in &lights {
                match (light.direction, car.direction) {
                    (Direction::South, Direction::North) => {
                        if car.y == light.y
                        {
                            if !light.green {
                                car_can_move = false;
                            }
                        }
                    }
                    (Direction::North, Direction::South) => {
                        if car.y == light.y
                        {
                            if !light.green {
                                car_can_move = false;
                            }
                        }
                    }
                    (Direction::East, Direction::West) => {
                        if car.x == light.x
                        {
                            if !light.green {
                                car_can_move = false;
                            }
                        }
                    }
                    (Direction::West, Direction::East) => {
                        if car.x + 10.0 == light.x
                        {
                            if !light.green {
                                car_can_move = false;
                            }
                        }
                    }
                    _ => {}
                }
            }

            if car_can_move {
                car.update();
            }
        }

        for car in &mut cars {
            match car.direction {
                Direction::North => match car.turn {
                    Turn::Left => {
                        if car.x == WINDOW_WIDTH / 2.0
                            && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH
                        {
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if (car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0) {
                            car.update_direction();
                        }
                    }
                    _ => {
                        car.update_direction();
                    }
                },
                Direction::South => match car.turn {
                    Turn::Left => {
                        if (car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH
                            && car.y == WINDOW_HEIGHT / 2.0)
                        {
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if (car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH
                            && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH)
                        {
                            car.update_direction();
                        }
                    }
                    _ => {
                        car.update_direction();
                    }
                },
                Direction::East => match car.turn {
                    Turn::Left => {
                        if car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0 {
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH && car.y == WINDOW_HEIGHT / 2.0
                        {
                            car.update_direction();
                        }
                    }
                    _ => {
                        car.update_direction();
                    }
                },
                Direction::West => match car.turn {
                    Turn::Left => {
                        if (car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH
                            && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH)
                        {
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH
                        {
                            car.update_direction();
                        }
                    }
                    _ => {
                        car.update_direction();
                    }
                },
            };
            // if can_move {
            //     car.update();
            // }
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

fn can_spawn(cars: &Vec<Car>, lane_pos: f32, min_distance: f32, dir_x: f32, dir_y: f32) -> bool {
    for car in cars {
        if dir_y == 0.0 {
            if (car.y - lane_pos).abs() < 1.0 {
                if dir_x > 0.0 && car.x < min_distance {
                    return false;
                }
                if dir_x < 0.0 && car.x > WINDOW_WIDTH - min_distance {
                    return false;
                }
            }
        } else if dir_x == 0.0 {
            if (car.x - lane_pos).abs() < 1.0 {
                if dir_y > 0.0 && car.y < min_distance {
                    return false;
                }
                if dir_y < 0.0 && car.y > WINDOW_HEIGHT - min_distance {
                    return false;
                }
            }
        }
    }
    true
}