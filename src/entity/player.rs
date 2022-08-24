use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use crate::asset_loader::TextureAssets;
use crate::entity::{Controllable, GameEntity, Hitbox, Motion};

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    texture: Res<TextureAssets>,
) {
    let gun = commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        texture: texture.basic_gun.clone(),
        ..default()
    })
        .insert(Name::new("Gun"))
        .insert(Gun)
        .id();
    commands.spawn_bundle(PlayerBundle {
        texture: texture.player.clone(),
        motion: Motion::new(0.1, 0.1),
        ..default()
    })
        .insert(GameEntity)
        .insert(Player)
        .insert(Name::new("Player"))
        .add_child(gun);
}

pub fn despawn_player(
    mut commands: Commands,
    q_ent: Query<Entity, With<Player>>,
) {
    for ent in q_ent.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

pub fn control_player(
    mut q_motion: Query<(&mut Motion, &Controllable), With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds() * 100.0;
    for (mut motion, cont) in q_motion.iter_mut() {
        if !cont.is_controllable { return; }
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
        }
        if key_right {
            motion.speed.x += motion.acc * delta;
        }
        if (key_left && key_right) || (!key_left && !key_right) {
            motion.speed.x -= motion.speed.x * motion.dcc * delta.clamp(0.0, 0.9);
        }
        motion.speed.y = motion.speed.y.clamp(-3., 3.);
        motion.speed.x = motion.speed.x.clamp(-3., 3.);
    }
}

pub fn move_gun(
    mut cursor_event_reader: EventReader<CursorMoved>,
    mut windows: ResMut<Windows>,
    mut q_gun: Query<&mut Transform, With<Gun>>,
    mut q_pl: Query<&mut Sprite, Or<(&Player, &Gun)>>,
) {
    let mouse = match cursor_event_reader.iter().next() {
        None => return,
        Some(s) => s,
    };
    let mut gun_t = match q_gun.get_single_mut() {
        Ok(g) => g,
        Err(_) => return,
    };
    let window = match windows.get_primary_mut() {
        None => return,
        Some(s) => s,
    };
    let pos_x = mouse.position.x - window.width() / 2.;
    let pos_y = mouse.position.y - window.height() / 2.;
    let rotation = (pos_y /pos_x).atan();
    gun_t.rotation = Quat::from_rotation_z(rotation);
    for mut sprite in q_pl.iter_mut() {
        if pos_x < 0. {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
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
            controllable: Controllable {
                is_controllable: true
            },
            motion: Default::default(),
        }
    }
}