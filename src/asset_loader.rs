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