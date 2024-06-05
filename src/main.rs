// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::close_on_esc;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use std::io::Cursor;
use tower::GamePlugin;
use winit::window::Icon;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tower".to_string(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                    synchronous_pipeline_compilation: false,
                }),
            GamePlugin,
        ))
        .add_systems(Startup, set_window_icon)
        .add_systems(Update, close_on_esc)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        // Do it for all windows
        for window in windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    };
}
