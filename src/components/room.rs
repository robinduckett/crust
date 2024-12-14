use super::{
    debug::{draw_rect, draw_surface, DebugTextInfo},
    utils::{intersect_wrapped_rect, normalize_rect, point_in_wrapped_rect, world_wrap},
    CreaturesGizmos, DebugText,
};
use crate::formats::sfc::ClassRegistry;
use crate::{
    camera::main_camera::{mouse_pos_to_world, MainCamera},
    constants::WORLD_WIDTH,
    display::{get_viewport_rect, tileset::RenderTile},
    formats,
    formats::sfc::{DropStatus, RoomType},
};
use bevy::color::palettes::tailwind::RED_300;
use bevy::render::view::RenderLayers;
use bevy::sprite::Anchor;
use bevy::{
    color::palettes::tailwind::{GREEN_300, PURPLE_300, YELLOW_300},
    prelude::*,
};
use ops::FloatPow;
use std::sync::{Arc, Mutex};

pub const NUMBER_OF_TIMES_OF_DAY: usize = 5;
pub const MAX_AMOUNT_OPEN: u8 = 255;
pub const MAX_ROOM_AREA: u32 = 700000; // Biggest room
pub const WIND_PER_P: u8 = 2;
pub const DAYS_IN_SEASON: u8 = 4;
pub const SEASONS_IN_YEAR: usize = 4;
pub const MAX_AREA: u8 = 1;
pub const AIR_QUALITY: [u8; 4] = [190, 224, 0, 255];

pub const HEAT_SOURCE_DELTA: [[i8; NUMBER_OF_TIMES_OF_DAY]; SEASONS_IN_YEAR] = [
    [-3, -2, -1, 1, -4],   // Spring
    [1, 3, 5, 5, -1],      // Summer
    [-3, -3, -2, -3, -5],  // Autumn
    [-6, -6, -5, -8, -10], // Winter
];

pub const LIGHT_SOURCE_DELTA: [[i8; NUMBER_OF_TIMES_OF_DAY]; SEASONS_IN_YEAR] = [
    [0, 0, 0, 0, 0], // Spring
    [0, 0, 0, 0, 0], // Summer
    [0, 0, 0, 0, 0], // Autumn
    [0, 0, 0, 0, 0], // Winter
];

pub const RADIATION_SOURCE_DELTA: [[i8; NUMBER_OF_TIMES_OF_DAY]; SEASONS_IN_YEAR] = [
    [0, 0, 0, 0, 0], // Spring
    [0, 0, 0, 0, 0], // Summer
    [0, 0, 0, 0, 0], // Autumn
    [0, 0, 0, 0, 0], // Winter
];

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (render_gizmo_rooms).chain());
        app.init_gizmo_group::<CreaturesGizmos>();
        app.register_type::<Room>();
        app.register_type::<Simulata>();
        app.register_type::<RenderTile>();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<CreaturesGizmos>();
    config.line_width = 2.0;
    config.render_layers = RenderLayers::from_layers(&[2]);

    // load doc
    let buf = include_bytes!("../../assets/test.sfc");
    let mut registry = Arc::new(Mutex::new(ClassRegistry::empty()));
    let (_, doc) = formats::sfc::Doc::parse(buf, &mut registry).unwrap();
    let font = asset_server.load("fonts/MS Sans Serif.ttf");

    let debug_text_width = 150.0;
    let debug_text_height = 180.0;

    commands.spawn(DebugText::new(
        "Cursor".to_string(),
        "".to_string(),
        &font,
        debug_text_width,
        debug_text_height,
    ));

    let parent = commands
        .spawn((
            Name::new("Rooms"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    for room in doc.map.rooms.rooms {
        let room_rect: Rect = room.rect.clone().into();

        let room_id = commands
            .spawn((
                Room {
                    rect: room_rect,
                    room_id: room.room_id,
                    room_type: room.room_type.clone(),
                    ground: Vec::from(&room.surface_points),
                    visited: false,
                },
                Transform::from_xyz(room_rect.min.x, room_rect.min.y, 0.00001),
                Anchor::TopLeft,
                Visibility::Visible,
                Name::new(format!("Room:{}", room.room_id)),
            ))
            .with_child(DebugText::new(
                format!("r{}", room.room_id),
                format!("room id: {}", room.room_id),
                &font,
                debug_text_width,
                room_rect.height(),
            ))
            .id();

        commands.entity(parent).add_child(room_id);
    }
}

// Ambience
#[derive(Component, Reflect)]
pub struct Ambience {
    pub music_track: String,
}

// Simulata
#[derive(Component, Reflect)]
pub struct Simulata {
    pub id: i32,
    pub room_type: RoomType,
    pub floor_value: u8,
    // soil
    pub inorganic_nutrient: u8,
    pub organic_nutrient: u8,
    // air
    pub new_temperature: u8,
    pub temperature: u8,
    pub heat_source: i32,
    // pressure
    pub new_pressure: u8,
    pub pressure: u8,
    pub wind: Vec2,
    pub pressure_source: i32,
    // light
    pub light_level: u8,
    pub light_source: i32,
    // radiation
    pub radiation: u8,
    pub radiation_source: i32,

    pub drop_status: DropStatus,
}

impl Default for Simulata {
    fn default() -> Self {
        Simulata {
            id: -1,
            room_type: RoomType::Invalid,
            floor_value: 126,
            inorganic_nutrient: 0, // Nothing can grow
            organic_nutrient: 0,
            new_temperature: 127, // Room temperature
            temperature: 127,
            heat_source: 0,
            new_pressure: 127,
            pressure: 127,
            wind: Vec2::ZERO,
            pressure_source: 0,
            light_level: 127,
            light_source: 0,
            radiation: 127,
            radiation_source: 0,
            drop_status: DropStatus::AboveFloor,
        }
    }
}

impl Simulata {
    pub fn _get_temperature(&self) -> u8 {
        // Add wind chill [10 unit wind speed == 1 'C].

        let wind: f32 = (self.wind.x.squared() + self.wind.y.squared()).abs().sqrt();

        let temperature: i16 = self.temperature as i16 - (wind / 10.0) as i16;

        if temperature < 0 {
            0
        } else if temperature > 255 {
            255
        } else {
            temperature as u8
        }
    }
}

// Room

#[derive(Component, Reflect)]
#[require(Simulata)]
pub struct Room {
    pub room_id: u32,
    pub rect: Rect,
    pub room_type: RoomType,
    pub ground: Vec<Vec2>,
    pub visited: bool,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            room_id: 0,
            rect: Rect::default(),
            room_type: RoomType::Invalid,
            ground: vec![],
            visited: false,
        }
    }
}

pub fn room_number<'a, I>(x: i32, y: i32, rooms: I) -> i32
where
    I: IntoIterator<Item = &'a Room>,
{
    let x = world_wrap(x);
    let pt = Vec2::new(x as f32, y as f32);
    let mut index = -1;

    for room in rooms {
        if point_in_wrapped_rect(room.rect, pt) {
            index = room.room_id as i32;
            break;
        }
    }

    index
}

// Debug Stuff

pub fn render_gizmo_rooms(
    mut gizmos: Gizmos<CreaturesGizmos>,
    mut main_camera: Query<(&Camera, &GlobalTransform, &OrthographicProjection), With<MainCamera>>,
    window_query: Query<&Window>,
    mut rooms_query: Query<(&Room, &mut Transform), With<Room>>,
    mut debug_cursor_text: Query<&mut DebugTextInfo, With<DebugTextInfo>>,
) {
    let mut main_camera_lens =
        main_camera.transmute_lens_filtered::<(&Camera, &GlobalTransform), With<MainCamera>>();

    let world_position = mouse_pos_to_world(main_camera_lens.query(), window_query);

    let (camera, camera_transform, ortho) = main_camera.single_mut();

    let viewport_rect = get_viewport_rect(camera, camera_transform, ortho).unwrap();

    draw_rect(&mut gizmos, viewport_rect, 0.2, 1.0, PURPLE_300);

    let mut in_wrapped_rect = false;
    let mut current_room: Option<&Room> = None;

    let mut rooms: Vec<&Room> = vec![];
    let mut room_transforms: Vec<Mut<Transform>> = vec![];

    rooms_query.iter_mut().for_each(|(room, transform)| {
        rooms.push(room);
        room_transforms.push(transform);
    });

    for (index, room) in rooms.iter().enumerate() {
        let room_rect: Rect = room.rect;

        let half_width = (WORLD_WIDTH / 2.0) as i32;

        let normalize = {
            let min_x: i32 = viewport_rect.min.x as i32 - room_rect.min.x as i32;

            min_x >= half_width || min_x < -half_width
        };

        let normalized_viewport_rect = if normalize {
            normalize_rect(viewport_rect.as_irect())
        } else {
            viewport_rect.as_irect()
        };

        let normalized_room_rect = if normalize {
            normalize_rect(room_rect.as_irect())
        } else {
            room_rect.as_irect()
        };

        if intersect_wrapped_rect(&normalized_viewport_rect, &normalized_room_rect) {
            if room_transforms[index].translation.x > viewport_rect.max.x {
                // room should be visible, but it's on the other side of the screen
                room_transforms[index].translation.x -= WORLD_WIDTH;
            }

            if room_transforms[index].translation.x + room_rect.width() < viewport_rect.min.x {
                room_transforms[index].translation.x += WORLD_WIDTH;
            }

            let ul = Vec2::new(
                room_transforms[index].translation.x,
                room_transforms[index].translation.y,
            );
            let rb = ul + Vec2::new(room_rect.width(), room_rect.height());
            let wrap_rect = Rect::from_corners(ul, rb);

            draw_surface(&mut gizmos, wrap_rect, &room.ground, YELLOW_300);

            if point_in_wrapped_rect(wrap_rect, world_position) {
                draw_rect(&mut gizmos, wrap_rect, 0.1, 0.0, RED_300);
                in_wrapped_rect = true;

                if current_room.is_some() {
                    if current_room.unwrap().room_id != room.room_id {
                        current_room = Some(room);
                    }
                } else {
                    current_room = Some(room);
                }
            } else {
                draw_rect(&mut gizmos, wrap_rect, 0.0, 0.0, GREEN_300);
            }
        }
    }

    debug_cursor_text
        .iter_mut()
        .filter(|dct| dct.name == "Cursor")
        .for_each(|mut dct| {
            let current_room = if let Some(current_room) = current_room {
                current_room
            } else {
                &Room::default()
            };

            dct.message = format!(
                "wx: {} wy: {}\nrn: {} rid: {} type: {}\nroomrect: {:?}\nin_wrapped_rect?: {:?}",
                world_position.x as i32,
                world_position.y as i32,
                room_number(
                    world_position.x as i32,
                    world_position.y as i32,
                    rooms.clone(),
                ),
                current_room.room_id,
                current_room.room_type,
                current_room.rect,
                in_wrapped_rect
            );

            dct.translation.x = world_position.x + 16.0;
            dct.translation.y = world_position.y - 16.0;
        });
}
