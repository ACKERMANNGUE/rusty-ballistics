pub const GRAVITY: f32 = 9.81;
pub const AIR_RESISTANCE: f32 = 0.001;

pub const WORLD_SIZE: (f32, f32) = (5000.0, 5000.0);

pub const HZ: f32 = 144.0;
pub const DELTA_TIME: f32 = 1.0 / HZ;

pub const BULLET_COUNT: usize = 15;

pub const TRAIL_MAX_POINTS: usize = 300;

pub const TURBULENCE_MAX_X: f32 = 0.5;
pub const TURBULENCE_MAX_Y: f32 = 0.5;
pub const TURBULENCE_DELTA_MAX: f32 = 0.005; 

pub const MAX_BULLET_VELOCITY: f32 = 1000.0;

pub const EPSILON: f32 = 1e-6;

pub const ANGULAR_DAMPING: f32 = 0.2;
pub const ANGULAR_VELOCITY_STOP_THRESHOLD: f32 = 0.001;