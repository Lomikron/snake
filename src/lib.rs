#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod game;
mod palette;
mod snake;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SNAKE_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0x3e3a42,
        0x877286,
        0xf0b695,
        0xe9f5da])
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game_state").update();
}