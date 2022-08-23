mod entity;
mod camera;
mod menus;
mod loading;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "la-kill-em".to_string(),
        resizable: false,
        width: 1280.0,
        height: 720.0,
        ..default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(AudioPlugin);
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
