use std::cmp::min;
use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy_inspector_egui::{RegisterInspectable, Inspectable};
use crate::{AppState, GameState};
use crate::asset_loader::TextureAssets;
use crate::entity::{Controllable, GameEntity, Hitbox, Motion};


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct UsingGun;

// inside magazine, magazine size, bullet type it shoots
#[derive(Component, Inspectable)]
pub struct Gun(pub u32, pub u32, BulletType);

impl Gun {
    fn reload(&mut self, ammo: &mut Ammo) {
        let amount = min(ammo.0, self.1);
        ammo.remove(self.1 - self.0);
        self.0 = amount;
    }
}

#[derive(Component, Inspectable)]
pub struct Ammo(pub u32);

impl Ammo {
    fn remove(&mut self, amount: u32) {
        self.0 -= min(amount, self.0);
    }
}

// bullet direction, bullet type, bullet origin
#[derive(Component, Inspectable)]
pub struct Bullet(f32, BulletType, Vec3);

#[derive(Component, Inspectable, Clone)]
pub enum BulletType {
    Basic,
    Rocket,
}

impl Bullet {
    fn speed(&self) -> f32 {
        match self.1 {
            BulletType::Basic => 50.,
            BulletType::Rocket => 10.,
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Belt(i8);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game(GameState::Playing))
                .with_system(spawn_player_with_guns)
            )
            .add_system_set(SystemSet::on_update(AppState::Game(GameState::Playing))
                .with_system(control_player)
                .with_system(move_gun.after("change").label("gun"))
                .with_system(shoot.after("gun").after("change").label("shoot"))
                .with_system(move_bullet)
                .with_system(change_gun.label("change"))
                .with_system(manual_reload.after("change"))
            );
        app.add_system_set(SystemSet::on_enter(AppState::Menu)
            .with_system(despawn_player)
        );
        app.register_inspectable::<Belt>();
        app.register_inspectable::<Gun>();
        app.register_inspectable::<Ammo>();
        app.register_inspectable::<Bullet>();
        app.register_inspectable::<BulletType>();
    }
}

fn spawn_player_with_guns(
    mut commands: Commands,
    texture: Res<TextureAssets>,
) {
    let gun = commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(1., -1., 1.),
        texture: texture.basic_gun.clone(),
        ..default()
    })
        .insert(Name::new("Gun"))
        .insert(Gun(6, 6, BulletType::Basic))
        .insert(Ammo(36))
        .insert(UsingGun)
        .insert(Belt(0))
        .id();
    let rocket = commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(2., 0., 1.),
        texture: texture.rocket_gun.clone(),
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
        .insert(Name::new("Rocket Gun"))
        .insert(Gun(1, 1, BulletType::Rocket))
        .insert(Ammo(10))
        .insert(Belt(1))
        .id();
    commands.spawn_bundle(PlayerBundle {
        texture: texture.player.clone(),
        motion: Motion::new(0.1, 0.1),
        ..default()
    })
        .insert(GameEntity)
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Belt(0))
        .push_children(&[gun, rocket]);
}

fn despawn_player(
    mut commands: Commands,
    q_ent: Query<Entity, With<Player>>,
) {
    for ent in q_ent.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn manual_reload(
    keys: Res<Input<KeyCode>>,
    mut q_gun: Query<(&mut Gun, &mut Ammo), With<UsingGun>>,
) {
    if keys.just_pressed(KeyCode::R) {
        let (mut gun, mut ammo) = q_gun.single_mut();
        if ammo.0 == 0 {
            // TODO: play sound
            return;
        }
        gun.reload(&mut ammo)
    }
}

// TODO: update rotation
fn change_gun(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut q_gun: Query<(&mut Visibility, Entity, &Belt), With<Gun>>,
    mut q_belt: Query<&mut Belt, (With<Player>, Without<Gun>)>,
) {
    let mut belt = q_belt.single_mut();
    if keys.just_pressed(KeyCode::E) {
        belt.0 += 1;
    } else if keys.just_pressed(KeyCode::Q) {
        belt.0 -= 1;
    }
    belt.0 = belt.0.clamp(0, q_gun.iter().collect::<Vec<_>>().len() as i8 - 1);
    for (mut vis, ent, gun_belt) in q_gun.iter_mut() {
        if belt.0 == gun_belt.0 {
            commands
                .entity(ent)
                .insert(UsingGun);
            vis.is_visible = true;
        } else {
            commands
                .entity(ent)
                .remove::<UsingGun>();
            vis.is_visible = false;
        }
    }
}

fn control_player(
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

fn move_gun(
    mut cursor_event_reader: EventReader<CursorMoved>,
    mut windows: ResMut<Windows>,
    mut q_gun: Query<&mut Transform, With<UsingGun>>,
    mut q_pl: Query<&mut Sprite, Or<(&UsingGun, &Player)>>,
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
    let rotation = (pos_y / pos_x).atan();
    gun_t.rotation = Quat::from_rotation_z(rotation);
    for mut sprite in q_pl.iter_mut() {
        if pos_x < 0. {
            sprite.flip_x = true;
            if gun_t.translation.x > 0. {
                gun_t.translation.x = -gun_t.translation.x;
            }
        } else {
            sprite.flip_x = false;
            if gun_t.translation.x < 0. {
                gun_t.translation.x = -gun_t.translation.x;
            }
        }
    }
}

fn shoot(
    mut commands: Commands,
    texture: Res<TextureAssets>,
    input: Res<Input<MouseButton>>,
    mut q_gun: Query<(&Transform, &GlobalTransform, &Sprite, &mut Gun, &mut Ammo), With<UsingGun>>,
) {
    if input.just_pressed(MouseButton::Left) {
        let (tr, g_tr, spr, mut gun, mut ammo) = match q_gun.get_single_mut() {
            Ok(g) => g,
            Err(_) => return,
        };
        if gun.0 == 0 {
            if ammo.0 == 0 {
                // TODO: play sound
                return;
            }
            gun.reload(&mut ammo);
            return;
        }
        gun.0 -= 1;
        let texture = match gun.2 {
            BulletType::Basic => texture.basic_bullet.clone(),
            BulletType::Rocket => texture.rocket_bullet.clone(),
        };
        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: g_tr.translation() - Vec3::new(0., 0., 1.),
                rotation: tr.rotation,
                ..default()
            },
            sprite: Sprite {
                flip_x: spr.flip_x,
                ..default()
            },
            texture,
            ..default()
        })
            .insert(Bullet(tr.translation.x, gun.2.clone(), g_tr.translation()));
    }
}

fn move_bullet(
    mut commands: Commands,
    mut q_bullet: Query<(&mut Transform, &Bullet, Entity)>,
) {
    for (mut tf, bt, ent) in q_bullet.iter_mut() {
        let rot = tf.rotation.to_euler(EulerRot::XYZ).2;
        tf.translation.x += bt.0 * bt.speed() * rot.cos();
        tf.translation.y += bt.0 * bt.speed() * rot.sin();
        match bt.1 {
            BulletType::Basic => {
                if bt.2.distance(tf.translation) > 1000. {
                    commands.entity(ent).despawn();
                }
            }
            BulletType::Rocket => {
                if bt.2.distance(tf.translation) > 4000. {
                    commands.entity(ent).despawn();
                }
            }
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