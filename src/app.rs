use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Turn {
    Left,
    Right,
    front,
}

// pub enum special {
//     (color, Turn)
// }

pub struct Car {
    pub direction: Direction,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub color: Color,
}

pub struct TrafficLight {
    pub direction: Direction,
    pub x: f32,
    pub y: f32,
    pub green: bool,
}

impl Car {
    pub fn new(direction: Direction, x: f32, y: f32, dx: f32, dy: f32, color: Color) -> Self {
        Self {
            direction,
            x, 
            y, 
            dx, 
            dy, 
            color, 
        }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 50.0, 50.0, self.color);
        draw_rectangle_lines(self.x, self.y, 50.0, 50.0, 2.0, WHITE);
    }
}

impl TrafficLight {
    pub fn new(direction: Direction, x: f32, y: f32, start_green: bool) -> Self {
        Self {
            direction: direction,
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