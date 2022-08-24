mod entity;
mod camera;
mod menus;
mod loading;
mod asset_loader;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use crate::loading::LoadingPlugin;
use crate::asset_loader::AssetsPlugin;
use crate::camera::CameraPlugin;
use crate::entity::EntityPlugin;
use crate::menus::MenuPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(ImageSettings::default_nearest());
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

    // own plugins
    app.add_plugin(LoadingPlugin);
    app.add_plugin(AssetsPlugin);
    app.add_plugin(MenuPlugin);
    app.add_plugin(EntityPlugin);
    app.add_plugin(CameraPlugin);

    app.run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Preload,
    LoadingAssets,
    Loading,

    Menu,
    Game(GameState),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    Paused,
}
