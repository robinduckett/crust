pub mod creature_eye_view;
pub mod main_camera;

use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

use crate::{constants::WORLD_WIDTH, display::get_viewport_rect, state::GameState};
use creature_eye_view::CreatureEyeView;
use main_camera::MainCamera;

pub type Cameras<'a> = (
    Entity,
    &'a Camera,
    &'a GlobalTransform,
    &'a OrthographicProjection,
    Option<&'a MainCamera>,
    Option<&'a CreatureEyeView>,
);

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                creature_eye_view::toggle_creatures_eye_view
                    .run_if(input_just_pressed(KeyCode::KeyM)),
                ((wrap_cameras, creature_eye_view::move_creature_eye_view).chain())
                    .run_if(in_state(GameState::Running)),
                creature_eye_view::select_new_eye_view
                    .run_if(input_just_pressed(MouseButton::Left)),
            ),
        );
    }
}

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(0.0 * 144.0, 0.0 * 150.0, 0.0),
            ..Default::default()
        },
        MainCamera,
        RenderLayers::from_layers(&[0, 1, 2]),
    ));

    let size = Extent3d {
        width: 256,
        height: 256,
        ..Default::default()
    };

    let mut creature_eye_view_image: Image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };

    creature_eye_view_image.resize(size);

    creature_eye_view_image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let camera_eye_view_texture = images.add(creature_eye_view_image);

    // commands.spawn((
    //     Camera2d,
    //     Camera {
    //         // order: -1,
    //         target: RenderTarget::Image(camera_eye_view_texture.clone()),
    //         ..default()
    //     },
    //     OrthographicProjection {
    //         scale: 1.2,
    //         ..OrthographicProjection::default_2d()
    //     },
    //     CreatureEyeView,
    //     RenderLayers::from_layers(&[0, 1]),
    //     CameraTarget {
    //         target: Vec2::new(405.0, -150.0),
    //     },
    // ));

    // commands
    //     .spawn((
    //         Node {
    //             display: Display::Flex,
    //             position_type: PositionType::Relative,
    //             left: Val::Px(10.0),
    //             top: Val::Px(10.0),
    //             width: Val::Px(128.0),
    //             height: Val::Px(128.0),
    //             padding: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::Px(5.0), Val::Px(5.0)),
    //             ..Default::default()
    //         },
    //         BackgroundColor(GRAY_200.into()),
    //         BorderRadius::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
    //         CreatureEyeView,
    //         Name::new("CreatureEyeView"),
    //     ))
    //     .with_children(|parent| {
    //         parent
    //             .spawn((
    //                 ImageNode::new(camera_eye_view_texture.clone()),
    //                 BorderRadius::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
    //             ))
    //             .with_children(|parent| {
    //                 parent.spawn((ImageNode::new(asset_server.load("Images/bubb.s16#0")),));
    //             });
    //     });
}

pub fn wrap_cameras(
    mut camera_query: Query<(
        &Camera,
        &mut Transform,
        &GlobalTransform,
        &OrthographicProjection,
    )>,
) {
    camera_query
        .iter_mut()
        .for_each(|(camera, mut camera_transform, global_transform, ortho)| {
            let Some(viewport) = get_viewport_rect(camera, global_transform, ortho) else {
                return;
            };

            if viewport.min.x <= 0.0 - viewport.width() {
                camera_transform.translation.x += WORLD_WIDTH;
                camera_transform.translation.x = camera_transform.translation.x.floor();
            }

            if viewport.min.x >= WORLD_WIDTH {
                camera_transform.translation.x -= WORLD_WIDTH;
                camera_transform.translation.x = camera_transform.translation.x.floor();
            }
        });
}
