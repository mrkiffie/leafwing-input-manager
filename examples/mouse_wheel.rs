use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::DerefMut;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<CameraMovement>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, zoom_camera)
        .add_systems(Update, pan_camera.after(zoom_camera))
        .run();
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum CameraMovement {
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(DualAxis)]
    Pan,
    PanLeft,
    PanRight,
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        // This will capture the total continuous value, for direct use.
        .with_axis(CameraMovement::Zoom, MouseScrollAxis::Y)
        // This will return a binary button-like output.
        .with(CameraMovement::PanLeft, MouseScrollDirection::LEFT)
        .with(CameraMovement::PanRight, MouseScrollDirection::RIGHT)
        // Alternatively, you could model them as a continuous dual-axis input
        .with_dual_axis(CameraMovement::Pan, MouseScroll::default())
        // Or even a digital dual-axis input!
        .with_dual_axis(CameraMovement::Pan, MouseScroll::default().digital());
    commands.spawn(Camera2d).insert(input_map);

    commands.spawn((
        Sprite::default(),
        Transform::from_scale(Vec3::new(100., 100., 1.)),
    ));
}

fn zoom_camera(query: Single<(&mut Projection, &ActionState<CameraMovement>), With<Camera2d>>) {
    const CAMERA_ZOOM_RATE: f32 = 0.05;

    let (mut camera_projection, action_state) = query.into_inner();
    // Here, we use the `action_value` method to extract the total net amount that the mouse wheel has travelled
    // Up and right axis movements are always positive by default
    let zoom_delta = action_state.value(&CameraMovement::Zoom);

    // We want to zoom in when we use mouse wheel up,
    // so we increase the scale proportionally
    // Note that the projection's scale should always be positive (or our images will flip)
    match camera_projection.deref_mut() {
        Projection::Orthographic(orthographic_projection) => {
            orthographic_projection.scale *= 1. - zoom_delta * CAMERA_ZOOM_RATE;
        }
        _ => unreachable!(),
    }
}

fn pan_camera(query: Single<(&mut Transform, &ActionState<CameraMovement>), With<Camera2d>>) {
    const CAMERA_PAN_RATE: f32 = 10.;

    let (mut camera_transform, action_state) = query.into_inner();

    // When using the `MouseScrollDirection` type, mouse wheel inputs can be treated like simple buttons
    if action_state.pressed(&CameraMovement::PanLeft) {
        camera_transform.translation.x -= CAMERA_PAN_RATE;
    }

    if action_state.pressed(&CameraMovement::PanRight) {
        camera_transform.translation.x += CAMERA_PAN_RATE;
    }
}
