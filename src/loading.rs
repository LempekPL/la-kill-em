use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;
use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Preload);
        app.add_system_set(SystemSet::on_update(AppState::Preload)
            .with_system(preload)
        );
        app.add_system_set(SystemSet::on_update(AppState::Loading)
            .with_system(setup)
        );
    }
}

#[derive(Component)]
struct LoadingText;

fn preload(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    mut wi_params: ResMut<WorldInspectorParams>
) {
    commands.spawn_bundle(Camera2dBundle::default())
        .insert(UiCameraConfig { show_ui: true });
    // loading text
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: UiColor(Color::BLACK),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            text: Text::from_section(
                "LOADING",
                TextStyle {
                    font: asset_server.load("fonts/OpenSans-Bold.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        });
    }).insert(LoadingText);
    // start loading assets
    app_state.set(AppState::LoadingAssets).unwrap();
    if !cfg!(debug_assertions) {
        wi_params.enabled = false;
    }
}

fn setup(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    loading_text_query: Query<Entity, With<LoadingText>>,
) {
    // remove loading text
    let loading_text_entity = loading_text_query.single();
    commands.entity(loading_text_entity).despawn_recursive();
    // move user to main menu
    app_state.set(AppState::Menu).unwrap();
}