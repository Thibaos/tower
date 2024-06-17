use bevy::math;
use bevy::window::CursorGrabMode;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::egui::emath::ease_in_ease_out;
#[cfg(not(target_family = "wasm"))]
use bevy_hanabi::{EffectAsset, EffectSpawner, ParticleEffectBundle};
use bevy_rapier3d::prelude::{Collider, ExternalImpulse, KinematicCharacterController, RigidBody};
use smooth_bevy_cameras::controllers::orbit::OrbitCameraController;

use crate::components::AttackController;
#[cfg(not(target_family = "wasm"))]
use crate::data::effects::new_effect_asset;
use crate::interpolation_functions::ease_out_expo;
use crate::{
    components::{CharacterDash, Player, PlayerMesh, ShotProjectile},
    data::bundles::{PlayerBundle, PlayerMeshBundle, ThirdPersonCameraBundle},
    interpolation_functions::lerp,
    GameState,
};

/// Spawns the `Camera3dBundle` and the player to be controlled
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    #[cfg(not(target_family = "wasm"))] mut effects: ResMut<Assets<EffectAsset>>,
) {
    let player_transform = Transform::from_xyz(0., 1., -10.);

    #[cfg(not(target_family = "wasm"))]
    let effect = new_effect_asset();
    #[cfg(not(target_family = "wasm"))]
    let effect_handle = effects.add(effect);

    dbg!("{effect_handle}");

    // spawn player
    let mut player = commands.spawn(PlayerBundle::new(player_transform));

    #[cfg(not(target_family = "wasm"))]
    {
        player.with_children(|node| {
            node.spawn(ParticleEffectBundle::new(effect_handle))
                .insert(Name::new("player_dash_effect"));
        });
    }

    let player_id = player.id();

    // player mesh
    commands
        .spawn(PlayerMeshBundle::new(&mut meshes, &mut materials))
        .set_parent(player_id);

    // following camera
    commands
        .spawn(ThirdPersonCameraBundle::new(
            Vec3::new(0., 1., -10.),
            Vec3::new(0., 0.6, 0.),
            Vec3::Y,
        ))
        .set_parent(player_id);
}

/// Handles keyboard input and movement, including a dash animation when it is started
fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut windows_query: Query<&mut Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut KinematicCharacterController, &mut CharacterDash), With<Player>>,
    mut player_shape_query: Query<&mut Transform, With<PlayerMesh>>,
    mut camera_controller: Query<&Transform, (Without<PlayerMesh>, With<OrbitCameraController>)>,
) {
    const BASE_SPEED: f32 = 0.04;
    const MAX_SPEED: f32 = BASE_SPEED * 6.;

    let camera_transform = camera_controller.single_mut();

    let window = windows_query.single_mut();

    let mut velocity = Vec3::ZERO;
    let local_z = camera_transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    let (mut player_controller, mut player_dash) = player_query.single_mut();
    let mut dash_requested = false;

    for key in keys.get_pressed() {
        match window.cursor.grab_mode {
            CursorGrabMode::None => (),
            _ => match key {
                KeyCode::KeyW => velocity += forward,
                KeyCode::KeyS => velocity -= forward,
                KeyCode::KeyA => velocity -= right,
                KeyCode::KeyD => velocity += right,
                KeyCode::Space => dash_requested = true,
                _ => (),
            },
        }
    }

    velocity = velocity.normalize_or_zero();

    if velocity.length() > 0.1 && dash_requested {
        player_dash.requested = true;
    }

    if player_dash.started {
        player_dash.progress +=
            (time.delta().as_secs_f64() * (1. / player_dash.duration_in_secs)) as f32;
        if player_dash.progress >= 1. {
            player_dash.started = false;
        }
        let speed = lerp(
            MAX_SPEED,
            BASE_SPEED * 2.,
            ease_out_expo(player_dash.progress),
        );
        player_controller.translation = Some(player_dash.direction * speed);
    } else {
        player_dash.direction = velocity;
        player_controller.translation = Some(velocity * BASE_SPEED);
    }

    let mut player_shape_transform = player_shape_query.single_mut();
    let target = player_shape_transform.translation + forward;
    player_shape_transform.look_at(target, Vec3::Y);
}

/// Shoot a ball with a dynamic rigidbody as a child entity from the player if the left mouse button is just pressed,
/// that auto-despawns after 5 seconds
fn player_attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_query: Query<(&Transform, &mut AttackController), With<Player>>,
    mut player_mesh_query: Query<&Transform, With<PlayerMesh>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let query_result = player_query.get_single_mut();
    if let Err(reason) = query_result {
        warn!("{}", reason);
        return;
    }

    let (player_transform, mut attack_controller) = query_result.unwrap();

    let mesh_result = player_mesh_query.get_single_mut();
    if let Err(reason) = mesh_result {
        warn!("{}", reason);
        return;
    }

    let mesh_transform = mesh_result.unwrap();

    if attack_controller.is_future_requested() || mouse.pressed(MouseButton::Left) {
        attack_controller.request_attack();
    }

    if attack_controller.consume_attack() {
        let position = player_transform.translation;
        let forward = mesh_transform.forward();
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(math::primitives::Sphere {
                    radius: 0.15,
                    ..default()
                }),
                material: materials.add(StandardMaterial::default()),
                transform: Transform::from_xyz(
                    position.x + forward.x,
                    position.y,
                    position.z + forward.z,
                )
                .looking_at(forward.into(), Vec3::Y),
                ..default()
            },
            Collider::ball(0.15),
            RigidBody::Dynamic,
            ExternalImpulse {
                impulse: forward.into(),
                ..default()
            },
            ShotProjectile,
        ));
    }
}

/// Start a dash animation for the player if it is requested and allowed
fn trigger_dash_on_request(
    mut entities_with_dash_ability: Query<(&mut CharacterDash, &Children), With<Player>>,
    #[cfg(not(target_family = "wasm"))] mut effect_spawners: Query<&mut EffectSpawner>,
    time: Res<Time>,
) {
    if entities_with_dash_ability.is_empty() {
        return;
    }

    for (mut dash_component, children) in entities_with_dash_ability.iter_mut() {
        if dash_component.requested && !dash_component.started {
            let now = time.elapsed_seconds_f64();
            if dash_component.last_update_in_secs + dash_component.cooldown_in_secs <= now {
                dash_component.last_update_in_secs = now;
                dash_component.requested = false;
                dash_component.started = true;
                dash_component.progress = 0.;

                #[cfg(not(target_family = "wasm"))]
                {
                    if let Ok(mut spawner) = effect_spawners.get_mut(children[0]) {
                        spawner.reset();
                    }
                }
            }
            dash_component.requested = false;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter::<GameState>(GameState::Playing), setup)
            .add_systems(
                Update,
                (player_movement, player_attack, trigger_dash_on_request)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
