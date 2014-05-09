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

pub static PIPE_MIN_SPAWN_X: f32 = GAME_WIDTH as f32 + 200.;
pub static PIPE_COLLISION_WIDTH: f32 = 140.;
pub static PIPE_X_GAP: f32 = 300.;
pub static PIPE_Y_GAP: f32 = 150.;
pub static PIPE_TOP_HEIGHT_MIN: f32 = 50.;
pub static PIPE_TOP_HEIGHT_MAX: f32 = GAME_HEIGHT as f32 - 50. - PIPE_Y_GAP - ground_height as f32;
pub static PIPE_X_VELOCITY: f32 = -1. * ground_width as f32 * GROUND_SPIN_FREQUENCY;

static ground_width: u32 = 864;
static ground_height: u32 = 96;
