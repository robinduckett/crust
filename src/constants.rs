use bevy::prelude::*;

pub const TILE_SIZE: Vec2 = Vec2::new(144.0, 150.0);
pub const WRAP_AROUND_WIDTH: f32 = 58.0;
pub const WRAP_AROUND_HEIGHT: f32 = 16.0;
pub const WORLD_WIDTH: f32 = WRAP_AROUND_WIDTH * TILE_SIZE.x;
pub const _WORLD_HEIGHT: f32 = WRAP_AROUND_HEIGHT * TILE_SIZE.y;
