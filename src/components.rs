#[cfg(debug_assertions)]
use bevy::log::info;
use bevy::prelude::{Component, Vec3};
use wasm_timer::SystemTime;

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
    pub last_update_in_secs: f64,
    pub cooldown_in_secs: f64,
    pub duration_in_secs: f64,
    pub direction: Vec3,
}

/// A component that allows an attack ability, with a specific cooldown and duration
#[derive(Component)]
pub struct AttackController {
    // should the attack be triggered now
    allowed: bool,
    // should the attack be triggered when the cooldown expires
    future_requested: bool,
    // the last triggered attack instant
    last_active_instant: SystemTime,
    // the cooldown
    cooldown_in_secs: f64,
    // unit factor of the allowed future request window
    request_window_factor: f64,
}

impl Default for AttackController {
    fn default() -> Self {
        Self {
            allowed: false,
            future_requested: false,
            last_active_instant: SystemTime::now(),
            cooldown_in_secs: 0.0,
            request_window_factor: 0.25,
        }
    }
}

impl AttackController {
    pub fn new(cooldown_in_secs: f64) -> Self {
        Self {
            cooldown_in_secs,
            ..Default::default()
        }
    }

    pub fn is_future_requested(&self) -> bool {
        self.future_requested
    }

    pub fn consume_attack(&mut self) -> bool {
        if self.allowed {
            self.reset();
            return true;
        }

        return false;
    }

    pub fn request_attack(&mut self) {
        let now = SystemTime::now();

        if self.allowed || now.cmp(&self.last_active_instant).is_le() {
            return;
        }

        let duration = now
            .duration_since(self.last_active_instant)
            .unwrap()
            .as_secs_f64();

        if duration >= self.cooldown_in_secs {
            self.allowed = true;
        } else if !self.future_requested
            && duration
                >= self.cooldown_in_secs - (self.cooldown_in_secs * self.request_window_factor)
        {
            #[cfg(debug_assertions)]
            {
                info!(
                    "attack requested after {}s, cooldown is {}s, max window is {}s",
                    duration,
                    self.cooldown_in_secs,
                    self.cooldown_in_secs * self.request_window_factor
                );
            }
            self.future_requested = true;
        }
    }

    fn reset(&mut self) {
        self.allowed = false;
        self.future_requested = false;
        self.last_active_instant = SystemTime::now();
    }
}

/// A marker component for the player's shape so we can query it separately from its parent
#[derive(Component, Default)]
pub struct PlayerMesh;

/// A marker component for the enemies game objects
#[derive(Component)]
pub struct Enemy;

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
