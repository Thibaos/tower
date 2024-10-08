use crate::loading::FontAssets;
use crate::GameState;

use bevy::prelude::*;

pub struct Start;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for Start {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_systems(OnEnter::<GameState>(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (click_play_button).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit::<GameState>(GameState::Menu), cleanup_menu);
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(120.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    justify: JustifyText::Left,
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },
                ..Default::default()
            });
        });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => state.set(GameState::Playing),
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands,
    button: Query<Entity, With<Button>>,
    camera: Query<Entity, With<Camera2d>>,
) {
    commands.entity(button.single()).despawn_recursive();
    commands.entity(camera.single()).despawn_recursive();
}
