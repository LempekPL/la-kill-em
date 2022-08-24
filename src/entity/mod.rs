pub(crate) mod player;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use crate::{AppState, GameState};
use crate::entity::player::{control_player, despawn_player, move_gun, spawn_player};


#[derive(Component)]
struct GameEntity;

#[derive(Component, Inspectable)]
pub struct Motion {
    #[inspectable(min = 0.001, max = 2.0)]
    pub acc: f32,
    #[inspectable(min = 0.001, max = 2.0)]
    pub dcc: f32,
    pub speed: Vec2,
}

#[derive(Component)]
pub struct Hitbox(Vec2);

#[derive(Component, Inspectable, Default)]
pub struct Controllable {
    pub is_controllable: bool,
}

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game(GameState::Playing))
                .with_system(spawn_player)
            )
            .add_system_set(SystemSet::on_update(AppState::Game(GameState::Playing))
                .with_system(entity_motion)
                .with_system(control_player)
                .with_system(move_gun)
            );
        app.add_system_set(SystemSet::on_enter(AppState::Menu)
            .with_system(despawn_player)
        );
        app.register_inspectable::<Motion>();
        app.register_inspectable::<Controllable>();
    }
}

fn entity_motion(
    mut q_motion: Query<(&mut Transform, &Motion), With<GameEntity>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds() * 100.0;
    for (mut movement, motion) in q_motion.iter_mut() {
        movement.translation.x += motion.speed.x * delta;
        movement.translation.y += motion.speed.y * delta;
    }
}


impl Motion {
    fn new(acc: f32, dcc: f32) -> Self {
        Self {
            acc,
            dcc,
            speed: Vec2::new(0.0, 0.0),
        }
    }
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            acc: 1.0,
            dcc: 1.0,
            speed: Vec2::new(0.0, 0.0),
        }
    }
}

impl Default for Hitbox {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

