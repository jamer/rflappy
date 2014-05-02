// vi: ts=4 sw=4

use constants::{GAME_WIDTH, GAME_HEIGHT};
use flappy_bird::FlappyBird;
use game::Game;
use window::Window;

pub fn main() {
	let game: ~Game = ~FlappyBird::new();
	let mut window: Window = Window::new("Flappy Bird", GAME_WIDTH, GAME_HEIGHT, game);
	window.event_loop();
}
