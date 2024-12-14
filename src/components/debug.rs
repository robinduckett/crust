use bevy::{color::palettes::css::WHITE, prelude::*, sprite::Anchor, text::TextBounds};
#[derive(Component)]
pub struct DebugTextInfo {
    pub name: String,
    pub message: String,
    pub translation: Vec2,
    pub width: f32,
    pub height: f32,
}

pub type DebugTextInfoType<'a> = (&'a mut Text2d, &'a mut Transform, &'a DebugTextInfo);
pub type ChangedDebugText = (With<DebugTextInfo>, Changed<DebugTextInfo>);

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct CreaturesGizmos;

#[derive(Bundle)]
pub struct DebugText {
    pub text: Text2d,
    pub text_layout: TextLayout,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_bounds: TextBounds,
    pub transform: Transform,
    pub anchor: Anchor,
    pub dct: DebugTextInfo,
}

impl DebugText {
    pub fn new(name: String, text: String, font: &Handle<Font>, width: f32, height: f32) -> Self {
        Self {
            text: Text2d::new(text.clone()),
            text_layout: TextLayout::default(),
            text_font: TextFont {
                font: font.clone(),
                font_size: 11.0,
                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
            },
            text_color: WHITE.into(),
            text_bounds: TextBounds::from(Vec2 {
                x: width,
                y: height,
            }),
            transform: Transform::from_xyz(5.0, -5.0 + height, 0.5),
            anchor: Anchor::TopLeft,
            dct: DebugTextInfo {
                name,
                message: text,
                translation: Vec2::ZERO,
                width,
                height,
            },
        }
    }
}

pub fn draw_rect(
    gizmos: &mut Gizmos<CreaturesGizmos>,
    rect: Rect,
    z: f32,
    margin: f32,
    color: Srgba,
) {
    let isometry = Isometry3d::new(
        Vec3::new(
            rect.min.x + rect.width() / 2.0 - margin,
            rect.min.y + rect.height() / 2.0 - margin,
            z,
        ),
        Quat::IDENTITY,
    );

    gizmos.rect(
        isometry,
        Vec2::new(rect.width() + margin * 2.0, rect.height() + margin * 2.0),
        color,
    );
}

pub fn draw_surface(
    gizmos: &mut Gizmos<CreaturesGizmos>,
    room_rect: Rect,
    ground_points: &[Vec2],
    color: Srgba,
) {
    let mut points: Vec<Vec2> = vec![];

    for point in ground_points.to_owned().iter() {
        let point = Vec2 {
            x: room_rect.min.x + point.x,
            y: (room_rect.min.y - (0.0 - point.y)),
        };

        points.push(point);
    }

    gizmos.linestrip_2d(points, color);
}

pub fn update_debug_text(mut debug_text_query: Query<DebugTextInfoType, ChangedDebugText>) {
    for (mut text, mut transform, debug_text) in &mut debug_text_query {
        if debug_text.name == "Cursor" {
            use bevy::text::TextSpanAccess;
            *text.write_span() = debug_text.message.clone();
            transform.translation.x = debug_text.translation.x;
            transform.translation.y = debug_text.translation.y;
        }
    }
}

pub fn add_debug_text_bg(
    mut commands: Commands,
    debug_text_query: Query<(Entity, &DebugTextInfo), Added<DebugTextInfo>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, debug_text) in &debug_text_query {
        commands.entity(entity).with_child((
            Mesh2d(meshes.add(Mesh::from(Rectangle::new(
                debug_text.width,
                debug_text.height,
            )))),
            MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.5))),
            Transform::from_xyz(
                debug_text.width / 2.0 - 5.0,
                -debug_text.height / 2.0 + 5.0,
                -0.0001,
            ),
            Anchor::TopLeft,
        ));
    }
}
