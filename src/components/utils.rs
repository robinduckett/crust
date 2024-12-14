use bevy::prelude::*;

use crate::constants::{TILE_SIZE, WORLD_WIDTH};

pub fn world_wrap_point(point: Vec2) -> Vec2 {
    Vec2::new(world_wrap(point.x as i32) as f32, point.y)
}

pub fn world_wrap(x: i32) -> i32 {
    x.rem_euclid(WORLD_WIDTH as i32)
}

pub fn world_wrap_tile(x: i32) -> u32 {
    x.rem_euclid((WORLD_WIDTH / TILE_SIZE.x) as i32) as u32
}

pub fn point_in_rect(rect: Rect, point: Vec2) -> bool {
    rect.contains(point)
}

pub fn point_in_wrapped_rect(rect: Rect, point: Vec2) -> bool {
    let wrapped_point = world_wrap_point(point);
    point_in_rect(rect, wrapped_point)
}

pub fn normalize(x: i32) -> i32 {
    let half_width = (WORLD_WIDTH / 2.0) as i32;
    let mut x = x;

    if x >= half_width {
        x -= half_width
    } else if x < half_width {
        x += half_width
    }

    x
}

#[allow(dead_code)]
pub fn normalize_point(point: Vec2) -> Vec2 {
    Vec2::new(normalize(point.x as i32) as f32, point.y)
}

pub fn normalize_rect(rect: IRect) -> IRect {
    IRect::new(
        normalize(rect.min.x),
        rect.min.y,
        normalize(rect.max.x),
        rect.max.y,
    )
}

pub fn intersect_wrapped_rect(view: &IRect, rect: &IRect) -> bool {
    let r1 = view;
    let r2 = rect;

    if r2.min.y >= r1.max.y || r1.min.y >= r2.max.y {
        return false;
    }

    if r1.max.x < WORLD_WIDTH as i32 && r2.max.x < WORLD_WIDTH as i32 {
        if r1.min.x >= r2.max.x || r2.min.x >= r1.max.x {
            return false;
        }
    } else if r1.max.x >= WORLD_WIDTH as i32 && r2.max.x >= WORLD_WIDTH as i32 {
        return true;
    } else if r1.max.x >= WORLD_WIDTH as i32 {
        let r1r = r1.max.x - WORLD_WIDTH as i32;
        if r2.min.x >= r1r && r2.max.x <= r1.min.x {
            return false;
        }
    } else {
        let r2r = r2.max.x - WORLD_WIDTH as i32;
        if r1.min.x >= r2r && r1.max.x <= r2.min.x {
            return false;
        }
    }

    true
}
