use bevy::input::mouse::{MouseMotion, MouseScrollUnit};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};
use smooth_bevy_cameras::LookAngles;
use smooth_bevy_cameras::{
    controllers::orbit::{ControlEvent, OrbitCameraController},
    LookTransform,
};

use crate::components::{Player, ZoomLevel};
use crate::GameState;

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    if window.cursor.grab_mode == CursorGrabMode::None {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_windows.single_mut();
    toggle_grab_cursor(&mut window);
}

fn control_system(
    time: Res<Time>,
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&mut LookTransform, &Transform, &mut ZoomLevel), Without<Player>>,
    player_position: Query<&Transform, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let cameras_result = cameras.get_single_mut();
    if cameras_result.is_err() {
        warn!("{}", cameras_result.unwrap_err());
        return;
    }

    let (mut transform, scene_transform, mut zoom) = cameras_result.unwrap();

    let mut look_angles = LookAngles::from_vector(-transform.look_direction().unwrap());
    let mut radius_scalar = 1.0;

    let mut max = 15.;
    let min = 0.1;

    let dt = time.delta_seconds();
    for event in events.read() {
        match event {
            ControlEvent::Orbit(delta) => {
                look_angles.add_yaw(dt * -delta.x);
                look_angles.add_pitch(dt * delta.y);
            }
            ControlEvent::TranslateTarget(delta) => {
                let right_dir = scene_transform.rotation * -Vec3::X;
                let up_dir = scene_transform.rotation * Vec3::Y;
                transform.target += dt * delta.x * right_dir + dt * delta.y * up_dir;
            }
            ControlEvent::Zoom(scalar) => {
                // radius scale is 1.2 if camera is zooming out, 0.8 if zooming in
                radius_scalar *= scalar;
                // update zoom component to keep track of the target zoom level
                zoom.0 = (zoom.0 * scalar).clamp(min, max);
            }
        }
    }

    look_angles.assert_not_looking_up();

    let ray_dir = -scene_transform.forward();
    let ray_origin = player_position.single().translation;

    // if there is a wall between the camera and the player,
    // set the maximum radius to the distance of the wall from the player
    if let Some((_, toi)) = rapier_context.cast_ray(
        ray_origin,
        ray_dir.into(),
        max,
        true,
        QueryFilter::only_fixed(),
    ) {
        max = toi - 1.;
    }

    let mut new_radius = (radius_scalar * transform.radius()).clamp(min, max.max(min));
    // if the radius is smaller than the target zoom level but no event makes it zoom out, zoom the camera out
    if new_radius < zoom.0 {
        radius_scalar = 1.2;
        new_radius = (radius_scalar * transform.radius()).clamp(min, max.max(min));
    }
    transform.eye = transform.target + new_radius * look_angles.unit_vector();
}

fn orbit_input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    controllers: Query<&OrbitCameraController>,
) {
    // Can only control one camera at a time.
    let Some(controller) = controllers.iter().find(|c| c.enabled) else {
        warn!("Could not find orbit camera controller for orbit_input_map.");
        return;
    };

    let OrbitCameraController {
        mouse_rotate_sensitivity,
        mouse_translate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        cursor_delta += event.delta;
    }

    events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));

    if mouse_buttons.pressed(MouseButton::Right) {
        events.send(ControlEvent::TranslateTarget(
            mouse_translate_sensitivity * cursor_delta,
        ));
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_reader.read() {
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar *= scroll_amount.mul_add(-mouse_wheel_zoom_sensitivity, 1.0);
    }
    events.send(ControlEvent::Zoom(scalar));
}

fn cursor_grab(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_windows.single_mut();

    if mouse_btn.just_pressed(MouseButton::Right) {
        toggle_grab_cursor(&mut window);
    }
}

pub struct ThirdPersonPlugin;

impl Plugin for ThirdPersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControlEvent>()
            .add_systems(
                OnEnter::<GameState>(GameState::Playing),
                initial_grab_cursor,
            )
            .add_systems(
                Update,
                (control_system, orbit_input_map, cursor_grab).run_if(in_state(GameState::Playing)),
            );
    }
}
