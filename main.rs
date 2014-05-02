// vi: ts=4 sw=4

use constants::{gameWidth, gameHeight};
use native;
use window::Window;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}

fn main() -> () {
	let mut window: Window = Window::new(gameWidth, gameHeight);
	window.event_loop();
}
