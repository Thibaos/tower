use std::time::Duration;

use bevy::prelude::{Component, Vec3};

/// A component to store the health points of an entity
#[derive(Component, Debug)]
pub struct Health(pub u32);

/// A marker component for the player's game object
#[derive(Component, Default)]
pub struct Player;

/// A component that allows a dash movement ability, with a specific cooldown and duration
#[derive(Component, Default)]
pub struct CharacterDash {
    pub requested: bool,
    pub started: bool,
    pub progress: f32,
    pub last_update: Duration,
    pub cooldown_in_ms: u128,
    pub duration_in_ms: u128,
    pub direction: Vec3,
}

/// A marker component for the player's shape so we can query it separately from its parent
#[derive(Component, Default)]
pub struct PlayerMesh;

/// A marker component for the enemies game objects
#[derive(Component)]
pub struct Enemy;

/// A component for entities which should disappear after a certain amount of time
#[derive(Component)]
pub struct LifeSpan {
    pub birth: Duration,
    pub life_time: Duration,
}

/// A single value component to keep track of the zoom level of the third person camera
#[derive(Component, Debug, Default)]
pub struct ZoomLevel(pub f32);

/// A marker component for the player projectiles
#[derive(Component)]
pub struct ShotProjectile;

/// A marker component for the boss health bar UI
#[derive(Component)]
pub struct BossHealth;

/// A marker component for the level location of the entity
#[derive(Component)]
pub struct LevelLocation(pub u32);
