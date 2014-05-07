// vi: ts=4 sw=4

use main::main;
use native;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}
