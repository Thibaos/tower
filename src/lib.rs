mod components;
mod data;
mod entity;
mod interpolation_functions;
mod state;

use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use state::loading;
use state::menu;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            loading::Assets,
            smooth_bevy_cameras::LookTransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            #[cfg(not(target_family = "wasm"))]
            bevy_hanabi::HanabiPlugin,
            menu::Start,
            entity::character::PlayerPlugin,
            entity::camera::ThirdPersonPlugin,
            entity::levels::SpawnBasicPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
                // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
                // bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
                bevy_rapier3d::render::RapierDebugRenderPlugin::default(),
            ));
        }
    }
}
