use crate::prelude::{Player, RESOLUTION};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct GameCameraPlugin;

#[derive(Component)]
pub struct GameCamera;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            Self::spawn_camera.label("camera"),
        )
        .add_system_to_stage(
            //The camera should update after all other game systems to enforce determinism and prevent jittering
            CoreStage::PostUpdate,
            Self::camera_follow
        );
    }
}

impl GameCameraPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn_bundle(UiCameraBundle::default());

        let mut camera = OrthographicCameraBundle::new_2d();
        camera.orthographic_projection.left = -1.0 * RESOLUTION;
        camera.orthographic_projection.right = 1.0 * RESOLUTION;
        camera.orthographic_projection.top = 1.0;
        camera.orthographic_projection.bottom = -1.0;
        camera.orthographic_projection.scaling_mode = ScalingMode::None;

        commands.spawn_bundle(camera).insert(GameCamera);
    }

    fn camera_follow(
        player_query: Query<&Transform, With<Player>>,
        mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
    ) {
        let player_transform = player_query.single().translation;
        let mut camera_transform = camera_query.single_mut();

        camera_transform.translation.x = player_transform.x;
        camera_transform.translation.y = player_transform.y;
    }
}
