use crate::{wasm4::{self}, palette::set_draw_color};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Point,
    pub can_turn: bool,
    pub input_buffer: u8,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Point { x: 2, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 0 },
            ],
            direction: Point { x: 1, y: 0 },
            can_turn: true,
            input_buffer: 0,
        }
    }

    pub fn update(&mut self) -> Option<Point> {
        self.body.insert(
            0,
            Point {
                x: (self.body[0].x + self.direction.x) % 20,
                y: (self.body[0].y + self.direction.y) % 20,
            },
        );

        if self.body[0].x < 0 {
            self.body[0].x = 19;
        }

        if self.body[0].y < 0 {
            self.body[0].y = 19;
        }
        
        self.can_turn = true;
        if self.input_buffer != 0 {
            self.turn(self.input_buffer);
            self.input_buffer = 0;
        }
        self.body.pop()
    }

    pub fn is_dead(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .any(|&body_section| body_section == self.body[0])
    }

    pub fn draw(&self) {
        set_draw_color(0x34);
        for &Point { x, y } in self.body.iter() {
            wasm4::rect(x * 8, y * 8, 8, 8);
        }

        set_draw_color(0x4);
        wasm4::rect(self.body[0].x * 8, self.body[0].y * 8, 8, 8);
    }

    pub fn turn(&mut self, input: u8)  {
        if input & wasm4::BUTTON_RIGHT != 0 && self.direction.x == 0 {
            if self.can_turn {
                self.direction = Point { x: 1, y: 0 };
                self.can_turn = false;
            } else {
                self.input_buffer = input;
            }
        } 
        if input & wasm4::BUTTON_LEFT != 0 && self.direction.x == 0 {
            if self.can_turn {
                self.direction = Point { x: -1, y: 0 };
                self.can_turn = false;
            } else {
                self.input_buffer = input;
            }
        }
        if input & wasm4::BUTTON_UP != 0 && self.direction.y == 0 {
            if self.can_turn {
                self.direction = Point { x: 0, y: -1 };
                self.can_turn = false;
            } else {
                self.input_buffer = input;
            }
        }
        if input & wasm4::BUTTON_DOWN != 0 && self.direction.y == 0 {
            if self.can_turn {
                self.direction = Point { x: 0, y: 1 };
                self.can_turn = false;
            } else {
                self.input_buffer = input;
            }
        }
    }   
}
