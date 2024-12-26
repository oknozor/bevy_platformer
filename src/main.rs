use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod colliders;
mod game_flow;
mod ground_detection;
mod player;
mod walls;

fn main() {
    let window = WindowPlugin {
        primary_window: Some(Window {
            resizable: true,
            mode: WindowMode::Windowed,
            resolution: WindowResolution::new(1900., 900.),
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(window),
        )
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        // .add_systems(Update, adjust_camera_on_resize)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(game_flow::GameFlowPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Update, camera::camera_fit_inside_current_level)
        .run();
}
