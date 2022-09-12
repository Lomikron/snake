use crate::palette::set_draw_color;
use crate::snake::{Point, Snake};
use crate::wasm4::{self};
use fastrand::Rng;

const FRUIT_SPRITE: [u8; 16] = [
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    state: GameState,
    rng: Rng,
    points: u32,
    snake: Snake,
    frame_count: u32,
    prev_gamepad: u8,
    fruit: Point,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);
        Self {
            state: GameState::Playing,
            points: 0,
            snake: Snake::new(),
            frame_count: 0,
            prev_gamepad: 0,
            fruit: Point {
                x: rng.i32(0..20),
                y: rng.i32(0..20),
            },
            rng: rng,
        }
    }

    pub fn update(&mut self) {
        set_draw_color(0x12);
        wasm4::text(self.points.to_string(), 0,0);
        if self.state == GameState::GameOver {
            wasm4::text("Game over", 10, 10);
            wasm4::text("Space to reset", 10, 40);
            self.input();
        }
        if self.state == GameState::Playing {
            self.frame_count += 1;

            self.input();

            if self.frame_count % 15 == 0 {
                let dropped_pos = self.snake.update();

                if self.snake.body[0] == self.fruit {
                    if let Some(last_pos) = dropped_pos {
                        self.snake.body.push(last_pos);
                    }

                    self.fruit.x = self.rng.i32(0..20);
                    self.fruit.y = self.rng.i32(0..20);
                    self.points += 1;
                }

                if self.snake.is_dead() {
                    self.state = GameState::GameOver;
                }
            }
            self.snake.draw();

            set_draw_color(0x4320);
            wasm4::blit(
                &FRUIT_SPRITE,
                self.fruit.x * 8,
                self.fruit.y * 8,
                8,
                8,
                wasm4::BLIT_2BPP,
            );
        }
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if (just_pressed & wasm4::BUTTON_1 != 0) && self.state == GameState::GameOver {
            self.snake = Snake::new();
            self.points = 0;
            self.prev_gamepad = 0;
            self.fruit.x = self.rng.i32(0..20);
            self.fruit.y = self.rng.i32(0..20);
            self.state = GameState::Playing;
        }
        self.snake.turn(just_pressed);

        self.prev_gamepad = gamepad;
    }
}
