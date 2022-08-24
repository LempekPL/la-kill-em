use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::entity::player::Player;

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::Game(GameState::Playing))
                .with_system(camera_follow_player)
            );
    }
}

#[allow(clippy::type_complexity)]
fn camera_follow_player(
    mut q_camera_player: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&Transform, With<Player>>
    )>
) {
    let player = q_camera_player.p1();
    if let Ok(player) = player.get_single() {
        let (player_x, player_y) = (player.translation.x, player.translation.y);
        let mut q_cam = q_camera_player.p0();
        let mut camera = q_cam.single_mut();
        camera.translation.x = player_x;
        camera.translation.y = player_y;
    } else {
        let mut q_cam = q_camera_player.p0();
        let mut camera = q_cam.single_mut();
        camera.translation.x = 0.;
        camera.translation.y = 0.;
    }
}