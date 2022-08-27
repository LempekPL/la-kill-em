use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt, AssetCollection};
use crate::AppState;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/OpenSans-Regular.ttf")]
    pub os_regular: Handle<Font>,
    #[asset(path = "fonts/OpenSans-Bold.ttf")]
    pub os_bold: Handle<Font>,
    #[asset(path = "fonts/OpenSans-Italic.ttf")]
    pub os_italic: Handle<Font>,
    #[asset(path = "fonts/OpenSans-BoldItalic.ttf")]
    pub os_bold_italic: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "textures/buttons/start.png")]
    pub b_start: Handle<Image>,
    #[asset(path = "textures/buttons/start-pressed.png")]
    pub b_start_pressed: Handle<Image>,
    #[asset(path = "textures/buttons/quit.png")]
    pub b_quit: Handle<Image>,
    #[asset(path = "textures/buttons/quit-pressed.png")]
    pub b_quit_pressed: Handle<Image>,
    #[asset(path = "textures/title.png")]
    pub title: Handle<Image>,
    #[asset(path = "textures/guns/basic-gun.png")]
    pub basic_gun: Handle<Image>,
    #[asset(path = "textures/bullets/basic-bullet.png")]
    pub basic_bullet: Handle<Image>,
    #[asset(path = "textures/guns/rocket-gun.png")]
    pub rocket_gun: Handle<Image>,
    #[asset(path = "textures/bullets/rocket-bullet.png")]
    pub rocket_bullet: Handle<Image>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<TextureAssets>()
        );
    }
}