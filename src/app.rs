use macroquad::prelude::*;

pub const SPEED:f32 = 5.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Turn {
    Left,
    Right,
    Front,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Col {
    Darkblue,
    Pink,
    Gold,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Car {
    pub direction: Direction,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub check_move: bool,
    pub color: Color,
    pub turn: Turn,
}

pub struct TrafficLight {
    pub direction: Direction,
    pub x: f32,
    pub y: f32,
    pub green: bool,
}

impl Car {
    pub fn new(
        direction: Direction,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        check_move: bool,
        color: Color,
        turn: Turn,
    ) -> Self {
        Self {
            direction,
            x,
            y,
            dx,
            dy,
            check_move,
            color,
            turn,
        }
    }

    pub fn update(&mut self) {
        self.check_move = true;
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn update_direction(&mut self) {
        match self.turn {
            Turn::Left => match self.direction {
                Direction::North => {
                    self.dx = -SPEED;
                    self.dy = 0.0;
                },
                Direction::South => {
                    self.dx = SPEED;
                    self.dy = 0.0;
                },
                Direction::East => {
                    self.dx = 0.0;
                    self.dy = -SPEED;
                },
                Direction::West => {
                    self.dx = 0.0;
                    self.dy = SPEED;
                },
            },
            Turn::Right => match self.direction {
                Direction::North => {
                    self.dx = SPEED;
                    self.dy = 0.0;
                },
                Direction::South => {
                    self.dx = -SPEED;
                    self.dy = 0.0;
                },
                Direction::East => {
                    self.dx = 0.0;
                    self.dy = SPEED;
                },
                Direction::West => {
                    self.dx = 0.0;
                    self.dy = -SPEED;
                },
            },
            _ => {}
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 50.0, 50.0, self.color);
        draw_rectangle_lines(self.x, self.y, 50.0, 50.0, 2.0, WHITE);
    }
}

impl TrafficLight {
    pub fn new(direction: Direction, x: f32, y: f32, start_green: bool) -> Self {
        Self {
            direction,
            x,
            y,
            green: start_green,
        }
    }
    pub fn update(&mut self) {
        if !self.green {
            self.green = !self.green;
        }
    }

    pub fn draw(&self) {
        let color = if self.green { GREEN } else { RED };
        draw_circle(self.x + 20.0, self.y + 20.0, 12.0, color);
        draw_circle_lines(self.x + 20.0, self.y + 20.0, 12.0, 2.0, WHITE);
        draw_rectangle(self.x + 15.0, self.y + 32.0, 10.0, 20.0, BROWN);
    }
}

impl From<Col> for Color {
    fn from(col: Col) -> Self {
        match col {
            Col::Darkblue => Color::from_rgba(0, 0, 139, 255),
            Col::Pink => Color::from_rgba(255, 105, 180, 255),
            Col::Gold => Color::from_rgba(255, 215, 0, 255),
        }
    }
}