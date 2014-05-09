// vi: ts=4 sw=4

use constants::{GAME_WIDTH, GAME_HEIGHT};
use flappy_bird::FlappyBird;
use game::Game;
use rsfml::system::vector2::Vector2u;
use window::Window;

pub fn main() {
	let window_size = Vector2u {
		x: GAME_WIDTH,
		y: GAME_HEIGHT,
	};
	let game: ~Game = ~FlappyBird::new(window_size);
	let mut window: Window = Window::new("Flappy Bird", window_size, game);
	window.event_loop();
}
