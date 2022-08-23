use bevy::prelude::*;
use crate::AppState;
use crate::asset_loader::TextureAssets;
use crate::Keyframes::Scale;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Menu)
                .with_system(spawn_menu)
            );
    }
}

#[derive(Component)]
enum ButtonType {
    ToGame,
    ToQuit,
    ToMenu,
}

#[derive(Component)]
struct MenuUILayer;

fn spawn_menu(
    mut commands: Commands,
    texture: Res<TextureAssets>,
    assets: Res<Assets<Image>>,
) {
    let button_size = 10.;
    let menu = commands.spawn_bundle(NodeBundle {
        color: UiColor(Color::hex("99d9ea").unwrap()),
        style: Style {
            size: Size::new(Val::Percent(80.), Val::Percent(80.)),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    })
        .insert(MenuUILayer)
        .insert(Name::new("Menu"))
        .id();

    let title = assets.get(&texture.title).unwrap();
    let title = commands.spawn_bundle(ImageBundle {
        image: UiImage(texture.title.clone()),
        style: Style {
            size: Size::new(
                Val::Px(title.texture_descriptor.size.width as f32 * 2.),
                Val::Px(title.texture_descriptor.size.height as f32 * 2.),
            ),
            ..default()
        },
        ..default()
    })
        .id();

    // buttons
    let start_button = assets.get(&texture.b_start).unwrap();
    let start_button = commands.spawn_bundle(ButtonBundle {
        image: UiImage(texture.b_start.clone()),
        style: Style {
            size: Size::new(
                Val::Px(start_button.texture_descriptor.size.width as f32 * button_size),
                Val::Px(start_button.texture_descriptor.size.height as f32 * button_size),
            ),
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        ..default()
    })
        .insert(ButtonType::ToGame)
        .insert(Name::new("Start button"))
        .id();
    let quit_button = assets.get(&texture.b_start).unwrap();
    let quit_button = commands.spawn_bundle(ButtonBundle {
        image: UiImage(texture.b_quit.clone()),
        style: Style {
            size: Size::new(
                Val::Px(quit_button.texture_descriptor.size.width as f32 * button_size),
                Val::Px(quit_button.texture_descriptor.size.height as f32 * button_size),
            ),
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        ..default()
    })
        .insert(ButtonType::ToQuit)
        .insert(Name::new("Quit button"))
        .id();
    commands.entity(menu).push_children(&vec![title, start_button, quit_button]);
}