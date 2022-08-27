use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::asset_loader::FontAssets;
use crate::entity::player::{Ammo, Gun, UsingGun};

#[derive(Component)]
pub struct AmmoText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game(GameState::Playing))
                .with_system(spawn_ui)
            )
            .add_system_set(SystemSet::on_update(AppState::Game(GameState::Playing))
                .with_system(update_ui.after("shoot"))
            )
            .add_system_set(SystemSet::on_exit(AppState::Game(GameState::Playing))
                .with_system(despawn_ui)
            );
    }
}

fn spawn_ui(
    mut commands: Commands,
    fonts: Res<FontAssets>,
) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            margin: UiRect::new(Val::Px(10.), Val::Undefined, Val::Undefined, Val::Undefined),
            ..default()
        },
        text: Text::from_sections(
            [
                TextSection::new(
                    "6/6",
                    TextStyle {
                        font: fonts.os_regular.clone(),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "\n",
                    TextStyle {
                        font: fonts.os_regular.clone(),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "36",
                    TextStyle {
                        font: fonts.os_regular.clone(),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
            ],
        ),
        ..default()
    })
        .insert(AmmoText);
}

fn despawn_ui(
    mut commands: Commands,
    q_ent: Query<Entity, With<AmmoText>>,
) {
    for ent in q_ent.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn update_ui(
    mut q_text: Query<&mut Text, With<AmmoText>>,
    q_gun: Query<(&Gun, &Ammo), With<UsingGun>>,
) {
    let (gun, ammo) = match q_gun.get_single() {
        Ok(i) => i as (&Gun, &Ammo),
        Err(_) => return,
    };
    let mut text = match q_text.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    text.sections[0].value = format!("{}/{}", gun.0, gun.1);
    text.sections[2].value = format!("{}", ammo.0);
}