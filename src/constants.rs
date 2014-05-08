// vi: ts=4 sw=4

pub static BIRD_IMAGE_FRAME_DURATION: f32 = 0.2;
pub static BIRD_IMAGE_FRAME_WIDTH: i32 = 85;
pub static BIRD_IMAGE_FRAME_HEIGHT: i32 = 60;
pub static BIRD_IMAGE_NFRAMES: i32 = 3;
pub static BIRD_JUMP_SET_Y_VELOCITY_PIXELS_PER_SECOND: f32 = -250.;
pub static BIRD_X_VELOCITY_PIXELS_PER_SECOND: f32 = GROUND_SPIN_FREQUENCY * ground_width as f32;
pub static BIRD_Y_ACCELERATION_PIXELS_PER_SECOND: f32 = 800.;
pub static GAME_WIDTH: u32 = 900;
pub static GAME_HEIGHT: u32 = 504;
pub static GROUND_SPIN_FREQUENCY: f32 = 0.2;

static ground_width: u32 = 864;