mod app;
use ::rand::{Rng, rng};
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
            false,
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
    let mut all_red_start: Option<f64> = None;
    let mut north_count = 0;
    let mut south_count = 0;
    let mut east_count = 0;
    let mut west_count = 0;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Up) {
            let mut rng = rng();
            let (col, turn) = colors[rng.random_range(0..colors.len())];
            let color = Color::from(col);
            let start_x = WINDOW_WIDTH / 2.0;
            if can_spawn(&cars, start_x, 50.0, 0.0, -SPEED) {
                north_count += 1;
                cars.push(Car::new(
                    Direction::North,
                    start_x,
                    WINDOW_HEIGHT + LANE_WIDTH,
                    0.0,
                    -SPEED,
                    true,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let mut rng = rng();
            let (col, turn) = colors[rng.random_range(0..colors.len())];
            let color = Color::from(col);
            let start_x = WINDOW_WIDTH / 2.0 - LANE_WIDTH;
            if can_spawn(&cars, start_x, 50.0, 0.0, SPEED) {
                south_count += 1;
                cars.push(Car::new(
                    Direction::South,
                    start_x,
                    -LANE_WIDTH,
                    0.0,
                    SPEED,
                    true,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let mut rng = rng();
            let (col, turn) = colors[rng.random_range(0..colors.len())];
            let color = Color::from(col);
            let start_y = WINDOW_HEIGHT / 2.0;
            if can_spawn(&cars, start_y, 50.0, SPEED, 0.0) {
                east_count += 1;
                cars.push(Car::new(
                    Direction::East,
                    -LANE_WIDTH,
                    start_y,
                    SPEED,
                    0.0,
                    true,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let mut rng = rng();
            let (col, turn) = colors[rng.random_range(0..colors.len())];
            let color = Color::from(col);
            let start_y = WINDOW_HEIGHT / 2.0 - LANE_WIDTH;
            if can_spawn(&cars, start_y, 50.0, -SPEED, 0.0) {
                west_count += 1;
                cars.push(Car::new(
                    Direction::West,
                    WINDOW_WIDTH + LANE_WIDTH,
                    start_y,
                    -SPEED,
                    0.0,
                    true,
                    color,
                    turn,
                ));
            }
        }

        if is_key_pressed(KeyCode::R) {
            let mut rng = rng();
            let direction = match rng.random_range(0..4) {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                _ => Direction::West,
            };

            let (start_x, start_y, velocity_x, velocity_y) = match direction {
                Direction::North => (WINDOW_WIDTH / 2.0, WINDOW_HEIGHT + LANE_WIDTH, 0.0, -SPEED),
                Direction::South => (WINDOW_WIDTH / 2.0 - LANE_WIDTH, -LANE_WIDTH, 0.0, SPEED),
                Direction::East => (-LANE_WIDTH, WINDOW_HEIGHT / 2.0, SPEED, 0.0),
                Direction::West => (
                    WINDOW_WIDTH + LANE_WIDTH,
                    WINDOW_HEIGHT / 2.0 - LANE_WIDTH,
                    -SPEED,
                    0.0,
                ),
            };

            let (col, turn) = colors[rng.random_range(0..colors.len())];
            let color = Color::from(col);

            if can_spawn(&cars, start_y, 50.0, velocity_x, velocity_y) {
                cars.push(Car::new(
                    direction, start_x, start_y, velocity_x, velocity_y, true, color, turn,
                ));
            }
        }

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

        if now - last_change >= 1.0 {
            if all_red_start.is_none() {
                for light in &mut lights {
                    light.green = false;
                }
                all_red_start = Some(now);
            } else if now - all_red_start.unwrap() >= 0.5 {
                // let random_index = rand::gen_range(0, lights.len());
                // lights[random_index].update();
                println!(
                    "{}, {}, {}, {}",
                    north_count, south_count, east_count, west_count
                );
                if north_count > south_count && north_count > east_count && north_count > west_count
                {
                    lights[1].update();
                } else if south_count > north_count
                    && south_count > east_count
                    && south_count > west_count
                {
                    lights[0].update();
                } else if east_count > north_count
                    && east_count > south_count
                    && east_count > west_count
                {
                    lights[3].update();
                } else if west_count > north_count
                    && west_count > south_count
                    && west_count > east_count
                {
                    lights[2].update();
                } else {
                    let random_index = rand::gen_range(0, lights.len());
                    lights[random_index].update();
                }

                last_change = now;
                all_red_start = None;
            }
        }

        for i in 0..cars.len() {
            let mut car_can_move = true;

            for light in &lights {
                match (light.direction, cars[i].direction) {
                    (Direction::South, Direction::North) => {
                        if cars[i].y == light.y {
                            if !light.green {
                                cars[i].check_move = false;
                                car_can_move = false;
                            }
                        }
                    }
                    (Direction::North, Direction::South) => {
                        if cars[i].y == light.y {
                            if !light.green {
                                cars[i].check_move = false;
                                car_can_move = false;
                            }
                        }
                    }
                    (Direction::East, Direction::West) => {
                        if cars[i].x == light.x {
                            if !light.green {
                                cars[i].check_move = false;
                                car_can_move = false;
                            }
                        }
                    }
                    (Direction::West, Direction::East) => {
                        if cars[i].x + 10.0 == light.x {
                            if !light.green {
                                cars[i].check_move = false;
                                car_can_move = false;
                            }
                        }
                    }
                    _ => {}
                }
            }

            for j in 0..cars.len() {
                if i == j {
                    continue;
                }
                let front_car = &cars[j];

                match cars[i].direction {
                    Direction::North => {
                        if front_car.direction == Direction::North
                            && front_car.y < cars[i].y
                            && cars[i].y - front_car.y < 90.0
                            && front_car.y >= lights[1].y
                        {
                            car_can_move = false;
                        }
                    }
                    Direction::South => {
                        if front_car.direction == Direction::South
                            && front_car.y > cars[i].y
                            && front_car.y - cars[i].y < 90.0
                            && front_car.y <= lights[0].y
                        {
                            car_can_move = false;
                        }
                    }
                    Direction::East => {
                        if front_car.direction == Direction::East
                            && front_car.x > cars[i].x
                            && front_car.x - cars[i].x < 90.0
                            && front_car.x <= lights[3].x
                        {
                            car_can_move = false;
                        }
                    }
                    Direction::West => {
                        if front_car.direction == Direction::West
                            && front_car.x < cars[i].x
                            && cars[i].x - front_car.x < 90.0
                            && front_car.x >= lights[2].x
                        {
                            car_can_move = false;
                        }
                    }
                }
            }

            if car_can_move {
                cars[i].update();
            }
        }

        for car in &mut cars {
            let mut should_decrement = false;

            match car.direction {
                Direction::North => match car.turn {
                    Turn::Left => {
                        if car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH && car.check == false
                        {
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0 && car.check == false{
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Front => {
                        if car.y <= WINDOW_HEIGHT / 2.0 - LANE_WIDTH && car.check == false{
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                },
                Direction::South => match car.turn {
                    Turn::Left => {
                        if car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH && car.y == WINDOW_HEIGHT / 2.0 && car.check == false
                        {
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH
                            && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH && car.check == false
                        {
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Front => {
                        if car.y >= WINDOW_HEIGHT / 2.0 + LANE_WIDTH && car.check == false{
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                },
                Direction::East => match car.turn {
                    Turn::Left => {
                        if car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0 && car.check == false{
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH && car.y == WINDOW_HEIGHT / 2.0 && car.check == false
                        {
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Front => {
                        if car.x >= WINDOW_WIDTH / 2.0 + LANE_WIDTH && car.check == false{
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                },
                Direction::West => match car.turn {
                    Turn::Left => {
                        if car.x == WINDOW_WIDTH / 2.0 - LANE_WIDTH
                            && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH && car.check == false
                        {
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Right => {
                        if car.x == WINDOW_WIDTH / 2.0 && car.y == WINDOW_HEIGHT / 2.0 - LANE_WIDTH && car.check == false
                        {
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                    Turn::Front => {
                        if car.x <= WINDOW_WIDTH / 2.0 - LANE_WIDTH && car.check == false{
                            should_decrement = true;
                            car.update_direction();
                        }
                    }
                },
            };

            if should_decrement {
                match car.direction {
                    Direction::North => {
                        if north_count > 0 {
                            car.check = true;
                            north_count -= 1;
                        }
                    }
                    Direction::South => {
                        if south_count > 0 {
                            car.check = true;
                            south_count -= 1;
                        }
                    }
                    Direction::East => {
                        if east_count > 0 {
                            car.check = true;
                            east_count -= 1;
                        }
                    }
                    Direction::West => {
                        if west_count > 0 {
                            car.check = true;
                            west_count -= 1;
                        }
                    }
                }
            }

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
