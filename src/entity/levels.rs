use bevy::{math, prelude::*};
use bevy_rapier3d::prelude::{Collider, Damping, ExternalForce, RapierContext, RigidBody};
use rand::Rng;

use crate::{
    components::{BossHealth, Enemy, Health, LevelLocation, Player, ShotProjectile},
    GameState,
};

pub const PI: f32 = 3.1415927;

const ROOM_WIDTH: f32 = 50.;
const HALF_ROOM_WIDTH: f32 = ROOM_WIDTH / 2.;
const ROOM_HEIGHT: f32 = ROOM_WIDTH / 3.;
const HALF_ROOM_HEIGHT: f32 = ROOM_HEIGHT / 2.;

#[inline]
fn y_offset(n: u32) -> f32 {
    n as f32 * ROOM_HEIGHT
}

#[derive(Resource, Debug)]
pub struct Level(pub u32);

impl Level {
    fn setup(
        &self,
        commands: &mut Commands,
        ground_mesh: Handle<Mesh>,
        wall_mesh: Handle<Mesh>,
        enemy_mesh: Handle<Mesh>,
        enemy_mat: Handle<StandardMaterial>,
        structure_mat: Handle<StandardMaterial>,
    ) {
        Self::spawn_structure(self, commands, ground_mesh, wall_mesh, structure_mat);
        Self::spawn_enemies(self, commands, enemy_mesh, enemy_mat);
        Self::spawn_decoration(self, commands);
    }

    fn spawn_structure(
        &self,
        commands: &mut Commands,
        ground_mesh: Handle<Mesh>,
        wall_mesh: Handle<Mesh>,
        structure_mat: Handle<StandardMaterial>,
    ) {
        let ground_collider = Collider::cuboid(HALF_ROOM_WIDTH, 0.01, HALF_ROOM_WIDTH);
        let wall_collider = Collider::cuboid(HALF_ROOM_WIDTH, HALF_ROOM_HEIGHT, 0.01);

        // ground
        commands.spawn((
            PbrBundle {
                mesh: ground_mesh.clone(),
                material: structure_mat.clone(),
                transform: Transform::from_xyz(0., y_offset(self.0), 0.),
                ..default()
            },
            ground_collider.clone(),
        ));

        // ceiling
        commands
            .spawn(PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0., y_offset(self.0) + ROOM_HEIGHT, 0.),
                    rotation: Quat::from_axis_angle(Vec3::X, PI),
                    ..default()
                },
                mesh: ground_mesh,
                material: structure_mat.clone(),
                ..default()
            })
            .insert(ground_collider);

        let create_wall = |x_offset: f32, z_offset: f32, rotation: Quat| {
            (
                PbrBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            x_offset,
                            y_offset(self.0) + HALF_ROOM_HEIGHT,
                            z_offset,
                        ),
                        rotation,
                        ..default()
                    },
                    mesh: wall_mesh.clone(),
                    material: structure_mat.clone(),
                    ..default()
                },
                wall_collider.clone(),
            )
        };

        // walls
        for wall in [
            create_wall(HALF_ROOM_WIDTH, 0., Quat::from_rotation_y(-PI / 2.)),
            create_wall(-HALF_ROOM_WIDTH, 0., Quat::from_rotation_y(PI / 2.)),
            create_wall(0., HALF_ROOM_WIDTH, Quat::from_rotation_x(PI)),
            create_wall(0., -HALF_ROOM_WIDTH, Quat::default()),
        ] {
            commands.spawn(wall);
        }
    }

    fn spawn_enemies(
        &self,
        commands: &mut Commands,
        enemy_mesh: Handle<Mesh>,
        enemy_mat: Handle<StandardMaterial>,
    ) {
        let transform = Transform::from_xyz(0., y_offset(self.0) + 2., 15.)
            .looking_at(y_offset(self.0) + Vec3::Y * 2., Vec3::Y);

        let entity = commands
            .spawn((
                PbrBundle {
                    mesh: enemy_mesh,
                    material: enemy_mat,
                    transform,
                    ..default()
                },
                Collider::cuboid(2., 2., 2.),
                RigidBody::Dynamic,
                ExternalForce::default(),
                Damping {
                    angular_damping: 5.,
                    ..default()
                },
                Health(1 + self.0),
                Enemy,
                LevelLocation(self.0),
            ))
            .id();

        let light = commands
            .spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            })
            .id();

        commands.entity(entity).add_child(light);
    }

    fn spawn_decoration(&self, commands: &mut Commands) {
        commands.spawn(PointLightBundle {
            transform: Transform::from_xyz(0., y_offset(self.0) + 10., 0.),
            point_light: PointLight {
                range: 40.,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    }
}

#[derive(Event)]
pub enum LevelEvent {
    Change(u32),
}

#[derive(Resource, Clone)]
struct GroundMesh(Handle<Mesh>);
#[derive(Resource, Clone)]
struct WallMesh(Handle<Mesh>);
#[derive(Resource, Clone)]
struct EnemyMesh(Handle<Mesh>);
#[derive(Resource, Clone)]
struct EnemyMat(Handle<StandardMaterial>);

#[derive(Component)]
struct LevelText;

fn setup_levels(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    level_index: Res<Level>,
) {
    let ground_mesh = GroundMesh(
        meshes.add(
            math::primitives::Plane3d::default()
                .mesh()
                .size(ROOM_WIDTH, ROOM_WIDTH),
        ),
    );
    let wall_mesh = WallMesh(meshes.add(math::primitives::Rectangle {
        half_size: Vec2::new(ROOM_WIDTH, ROOM_HEIGHT) / 2.0,
        ..default()
    }));

    let enemy_mesh = EnemyMesh(meshes.add(math::primitives::Cuboid {
        half_size: Vec3::ONE * 2.0,
    }));
    let enemy_mat = EnemyMat(materials.add(StandardMaterial {
        base_color: Color::rgba(1., 1., 1., 0.4),
        emissive: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        ..default()
    }));

    let structure_mat = materials.add(StandardMaterial {
        base_color: Color::Hsla {
            hue: 360. / 12. * level_index.0 as f32,
            saturation: 0.8,
            lightness: 0.4,
            alpha: 1.,
        },
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.insert_resource(ground_mesh.clone());
    commands.insert_resource(wall_mesh.clone());
    commands.insert_resource(enemy_mesh.clone());
    commands.insert_resource(enemy_mat.clone());

    Level(level_index.0).setup(
        &mut commands,
        ground_mesh.0,
        wall_mesh.0,
        enemy_mesh.0,
        enemy_mat.0,
        structure_mat,
    );

    commands
        // main_container node
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|main_container| {
            main_container
                // bottom_container node (70% width)
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(70.),
                        height: Val::Auto,
                        margin: UiRect::bottom(Val::Px(36.)),
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|bottom_container| {
                    bottom_container
                        // text_container node (100% width)
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Px(32.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|text_container| {
                            // text_content node holding the value of the current level
                            text_container.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "Level 1",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                },
                                LevelText,
                            ));
                        });
                })
                .with_children(|bottom_container| {
                    bottom_container
                        // health_bar_background
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Px(10.),
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            border_color: Color::BLACK.into(),
                            background_color: Color::rgba(0.3, 0.3, 0.4, 0.8).into(),
                            ..default()
                        })
                        .with_children(|health_bar_background| {
                            // health_bar node (100% width, 12px height) whose width should be bound to the enemy health
                            health_bar_background.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        ..default()
                                    },
                                    background_color: Color::rgba(0.6, 0.2, 0.2, 0.95).into(),
                                    ..default()
                                },
                                BossHealth,
                            ));
                        });
                });
        });
}

fn enemy_movement(time: &Res<Time>, enemy_external_force: &mut ExternalForce) {
    if time.elapsed_seconds() % 3. < 1. {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1. ..1.);
        let z: f32 = rng.gen_range(-1. ..1.);
        enemy_external_force.force = Vec3::new(x, 0., z) * 1024.;
    }
}

fn update_current_level(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut current_level: ResMut<Level>,
    mut commands: Commands,
    mut events_writer: EventWriter<LevelEvent>,
    mut enemy_query: Query<(
        Entity,
        &mut ExternalForce,
        &mut Health,
        &Enemy,
        &LevelLocation,
    )>,
    mut boss_health_query: Query<(&mut Style, &BossHealth)>,
    mut level_text_query: Query<(&mut Text, &LevelText)>,
    projectile_query: Query<(Entity, &ShotProjectile)>,
) {
    for (enemy_id, mut external_force, mut health, _, location) in enemy_query.iter_mut() {
        if location.0 != current_level.0 {
            return;
        }

        enemy_movement(&time, &mut external_force);

        // Iterate through all the contact pairs involving player projectiles and the enemy
        for (projectile_id, _) in &projectile_query {
            if rapier_context
                .contact_pair(enemy_id, projectile_id)
                .is_some()
            {
                let (mut boss_health_style, _) = boss_health_query.single_mut();
                let (mut level_text_style, _) = level_text_query.single_mut();

                commands.entity(projectile_id).despawn_recursive();
                health.0 -= 1;
                if health.0 == 0 {
                    commands.entity(enemy_id).despawn_recursive();
                    current_level.0 += 1;
                    events_writer.send(LevelEvent::Change(current_level.0));
                    projectile_query
                        .iter()
                        .for_each(|(id, _)| commands.entity(id).despawn_recursive());
                    boss_health_style.width = Val::Percent(100.);
                    boss_health_style.height = Val::Percent(100.);
                    let mut new_str = "Level ".to_owned();
                    new_str.push_str((current_level.0 + 1).to_string().as_str());
                    level_text_style.sections[0].value = new_str;
                    return;
                }

                boss_health_style.width =
                    Val::Percent(health.0 as f32 / (current_level.0 + 1) as f32 * 100.);
                boss_health_style.height = Val::Percent(100.);
            }
        }
    }
}

fn tp_player_on_level_change(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events_reader: EventReader<LevelEvent>,
    mut player_query: Query<(&mut Transform, &Player)>,
    ground_mesh: Res<GroundMesh>,
    wall_mesh: Res<WallMesh>,
    enemy_mesh: Res<EnemyMesh>,
    enemy_mat: Res<EnemyMat>,
) {
    for event in events_reader.read() {
        match event {
            LevelEvent::Change(index) => {
                let (mut transform, _) = player_query.single_mut();

                let structure_mat = materials.add(StandardMaterial {
                    base_color: Color::Hsla {
                        hue: 360. / 16. * *index as f32,
                        saturation: 0.8,
                        lightness: 0.4,
                        alpha: 1.,
                    },
                    perceptual_roughness: 0.9,
                    ..default()
                });

                Level(*index).setup(
                    &mut commands,
                    ground_mesh.0.clone(),
                    wall_mesh.0.clone(),
                    enemy_mesh.0.clone(),
                    enemy_mat.0.clone(),
                    structure_mat,
                );

                transform.translation = Vec3::new(
                    transform.translation.x,
                    transform.translation.y - y_offset(*index - 1) + y_offset(*index),
                    transform.translation.z,
                );
            }
        }
    }
}

pub struct SpawnBasicPlugin;

impl Plugin for SpawnBasicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>()
            .insert_resource(Level(0))
            .add_systems(OnEnter::<GameState>(GameState::Playing), setup_levels)
            .add_systems(
                Update,
                (update_current_level, tp_player_on_level_change)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
