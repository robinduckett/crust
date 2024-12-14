use bevy::{prelude::*, sprite::Anchor};

use crate::camera::Cameras;
use crate::components::utils::world_wrap_tile;
use crate::constants::{TILE_SIZE, WORLD_WIDTH, WRAP_AROUND_HEIGHT};

use super::get_viewport_rect;

#[derive(Component, Reflect)]
pub struct BackgroundTiles;

#[derive(Component, Reflect, Debug)]
pub struct RenderTile {
    pub camera_id: u32,
    pub index: u32,
    pub is_wrapping: bool,
}

pub fn render_background_wrapped(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Cameras>,
    mut rendered_tiles: Query<(&mut Transform, &mut Sprite, &RenderTile)>,
    background_tiles_query: Single<Entity, With<BackgroundTiles>>,
) {
    let background_tiles = *background_tiles_query;

    for (entity, camera, camera_transform, ortho, main_camera, creature_camera) in
        camera_query.iter()
    {
        let viewport = get_viewport_rect(camera, camera_transform, ortho).unwrap();

        let margin = viewport.inflate(200.0);
        let margin = Rect::new(
            margin.min.x / TILE_SIZE.x,
            margin.min.y / TILE_SIZE.y,
            margin.max.x / TILE_SIZE.x,
            margin.max.y / TILE_SIZE.y,
        )
        .as_irect();

        for j in margin.min.y..=margin.max.y {
            for i in margin.min.x..=margin.max.x {
                let x = i;
                let y = j;

                let tile_x = world_wrap_tile(x);
                let tile_y = 0 - y;

                if tile_y >= WRAP_AROUND_HEIGHT as i32 {
                    continue;
                }

                if tile_y < 0 {
                    continue;
                }

                let is_wrapping_left = x < 0;
                let is_wrapping_right = x >= WORLD_WIDTH as i32;
                let is_wrapping = is_wrapping_left || is_wrapping_right;

                let tile_index = tile_x * WRAP_AROUND_HEIGHT as u32 + tile_y as u32;

                let image: Handle<Image> =
                    asset_server.load(format!("sprites/Back.s16#{}", tile_index));

                let camera_name = if main_camera.is_some() {
                    "Main"
                } else if creature_camera.is_some() {
                    "Creature"
                } else {
                    "Unknown"
                };

                let current_tiles = rendered_tiles
                    .iter_mut()
                    .filter(|(_, _, rt)| {
                        rt.camera_id == entity.index() && rt.index == tile_index && !rt.is_wrapping
                    })
                    .collect::<Vec<_>>();

                if current_tiles.is_empty() {
                    let mut tile =
                        commands.spawn(spawn_tile(entity.index(), &image, tile_index, x, y, false));

                    let tile_id = tile
                        .insert(Name::new(format!("Tile:{}:{}", camera_name, tile_index)))
                        .id();

                    commands.entity(background_tiles).add_child(tile_id);
                } else {
                    for (mut transform, mut _sprite, rt) in current_tiles {
                        position_tile(x, y, &mut transform.translation);
                    }
                }

                if is_wrapping {
                    let current_tiles_wrapped = rendered_tiles
                        .iter_mut()
                        .filter(|(_, _, rt)| {
                            rt.camera_id == entity.index()
                                && rt.index == tile_index
                                && rt.is_wrapping
                        })
                        .collect::<Vec<_>>();

                    if current_tiles_wrapped.is_empty() {
                        let mut wrapping_tile = commands.spawn(spawn_tile(
                            entity.index(),
                            &image,
                            tile_index,
                            0 - tile_x as i32,
                            y,
                            true,
                        ));

                        let tile_id = wrapping_tile
                            .insert(Name::new(format!(
                                "WrapTile:{}:{}",
                                camera_name, tile_index
                            )))
                            .id();

                        commands.entity(background_tiles).add_child(tile_id);
                    } else {
                        for (mut transform, mut _sprite, rt) in current_tiles_wrapped {
                            position_tile(tile_x as i32, y, &mut transform.translation);
                        }
                    }
                }
            }
        }
    }
}

fn position_tile(tile_x: i32, tile_y: i32, translation: &mut Vec3) {
    let world_x = tile_x as f32 * TILE_SIZE.x;
    let world_y = tile_y as f32 * TILE_SIZE.y;
    let offset = Vec3::new(TILE_SIZE.x / 2.0, -(TILE_SIZE.y / 2.0), 0.0);
    let position = offset + Vec3::new(world_x, world_y, -0.01);

    if *translation != position {
        *translation = position;
    }
}

fn spawn_tile(
    camera_id: u32,
    image: &Handle<Image>,
    tile_index: u32,
    tile_x: i32,
    tile_y: i32,
    is_wrapping: bool,
) -> impl Bundle {
    let mut position = Vec3::new(0.0, 0.0, 0.0);
    position_tile(tile_x, tile_y, &mut position);

    (
        Sprite {
            image: image.clone(),
            ..Default::default()
        },
        Transform {
            translation: position,
            ..Default::default()
        },
        RenderTile {
            index: tile_index,
            camera_id,
            is_wrapping,
        },
        Anchor::TopLeft,
    )
}
