use bevy::{
    core_pipeline::{
        core_3d,
        tonemapping::{DebandDither, Tonemapping},
    },
    math::{self, Vec2},
    prelude::{
        default, Assets, Bundle, Camera, Camera3d, Color, GlobalTransform, Mesh, PbrBundle,
        Projection, ResMut, StandardMaterial, Transform, Vec3,
    },
    render::{
        camera::{CameraMainTextureUsages, CameraRenderGraph, Exposure},
        primitives::Frustum,
        view::{ColorGrading, VisibleEntities},
    },
};
use bevy_rapier3d::prelude::{CharacterLength, Collider, KinematicCharacterController, RigidBody};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use crate::components::{AttackController, CharacterDash, Player, PlayerMesh, ZoomLevel};

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    #[bundle()]
    pub pbr: PbrBundle,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub rigidbody: RigidBody,
    pub dash: CharacterDash,
    pub attack_controller: AttackController,
    pub marker: Player,
}

impl PlayerBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            pbr: PbrBundle {
                transform,
                ..default()
            },
            collider: Collider::cylinder(1., 0.5),
            controller: KinematicCharacterController {
                offset: CharacterLength::Absolute(0.1),
                autostep: None,
                snap_to_ground: Some(CharacterLength::Absolute(1.)),
                apply_impulse_to_dynamic_bodies: true,
                ..default()
            },
            rigidbody: RigidBody::KinematicVelocityBased,
            dash: CharacterDash {
                cooldown_in_secs: 1.2,
                duration_in_secs: 1.0,
                ..default()
            },
            attack_controller: AttackController::new(0.5),
            marker: Player,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerMeshBundle {
    #[bundle()]
    pub pbr: PbrBundle,
    pub marker: PlayerMesh,
}

impl PlayerMeshBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::Z, Vec3::Y),
                mesh: meshes.add(math::primitives::Capsule3d::default()),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 0.05,
                    metallic: 0.9,
                    ..default()
                }),
                ..default()
            },
            marker: PlayerMesh,
        }
    }
}

#[derive(Bundle)]
pub struct ThirdPersonCameraBundle {
    #[bundle()]
    pub orbit_camera: OrbitCameraBundle,
    #[bundle()]
    pub camera_3d: NoTransformCamera3dBundle,
    pub zoom_level: ZoomLevel,
}

impl Default for ThirdPersonCameraBundle {
    fn default() -> Self {
        Self {
            orbit_camera: OrbitCameraBundle::new(
                OrbitCameraController::default(),
                Vec3::default(),
                Vec3::default(),
                Vec3::Y,
            ),
            camera_3d: NoTransformCamera3dBundle::default(),
            zoom_level: ZoomLevel::default(),
        }
    }
}

impl ThirdPersonCameraBundle {
    pub fn new(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        Self {
            orbit_camera: OrbitCameraBundle::new(
                OrbitCameraController {
                    mouse_rotate_sensitivity: Vec2::new(0.2, 0.2),
                    ..default()
                },
                eye,
                target,
                up,
            ),
            zoom_level: ZoomLevel(5.),
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct NoTransformCamera3dBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: Projection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub global_transform: GlobalTransform,
    pub camera_3d: Camera3d,
    pub tonemapping: Tonemapping,
    pub dither: DebandDither,
    pub color_grading: ColorGrading,
    pub exposure: Exposure,
    pub main_texture_usages: CameraMainTextureUsages,
}

impl Default for NoTransformCamera3dBundle {
    fn default() -> Self {
        Self {
            camera_render_graph: CameraRenderGraph::new(core_3d::graph::Core3d),
            tonemapping: Tonemapping::ReinhardLuminance,
            camera: Camera::default(),
            projection: Projection::default(),
            visible_entities: VisibleEntities::default(),
            frustum: Frustum::default(),
            global_transform: GlobalTransform::default(),
            camera_3d: Camera3d::default(),
            color_grading: Default::default(),
            exposure: Default::default(),
            main_texture_usages: Default::default(),
            dither: DebandDither::Enabled,
        }
    }
}
