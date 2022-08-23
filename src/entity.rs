use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use crate::{AppState, GameState};
use crate::asset_loader::TextureAssets;

#[derive(Component)]
pub struct Player;

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
            );
        app.add_system_set(SystemSet::on_enter(AppState::Menu)
            .with_system(despawn_player)
        );
        app.register_inspectable::<Motion>();
        app.register_inspectable::<Controllable>();
    }
}

fn spawn_player(
    mut commands: Commands,
    texture: Res<TextureAssets>,
) {
    commands.spawn_bundle(PlayerBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..default()
        },
        texture: texture.player.clone(),
        motion: Motion::new(0.1, 0.1),
        ..default()
    })
        .insert(GameEntity)
        .insert(Player)
        .insert(Name::new("Player"));
}

fn despawn_player(
    mut commands: Commands,
    q_ent: Query<Entity, With<Player>>,
) {
    for ent in q_ent.iter() {
        commands.entity(ent).despawn();
    }
}

fn control_player(
    mut q_motion: Query<(&mut Motion, &mut Sprite)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds() * 100.0;
    for (mut motion, mut sprite) in q_motion.iter_mut() {
        let key_left = keys.pressed(KeyCode::A);
        let key_right = keys.pressed(KeyCode::D);
        let key_up = keys.pressed(KeyCode::W);
        let key_down = keys.pressed(KeyCode::S);
        // up down
        if key_down {
            motion.speed.y -= motion.acc * delta;
        }
        if key_up {
            motion.speed.y += motion.acc * delta;
        }
        if (key_down && key_up) || (!key_down && !key_up) {
            motion.speed.y -= motion.speed.y * motion.dcc * delta.clamp(0.0, 0.9);
        }
        // left right
        if key_left {
            motion.speed.x -= motion.acc * delta;
            sprite.flip_x = true;
        }
        if key_right {
            motion.speed.x += motion.acc * delta;
            sprite.flip_x = false;
        }
        if (key_left && key_right) || (!key_left && !key_right) {
            motion.speed.x -= motion.speed.x * motion.dcc * delta.clamp(0.0, 0.9);
        }
        motion.speed.y = motion.speed.y.clamp(-3., 3.);
        motion.speed.x = motion.speed.x.clamp(-3., 3.);
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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub hitbox: Hitbox,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub controllable: Controllable,
    pub motion: Motion,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            hitbox: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            controllable: Default::default(),
            motion: Default::default(),
        }
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

