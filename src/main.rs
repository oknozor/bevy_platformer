use crate::grid::translate_grid_coords_entities;
use crate::player::{move_player_from_input, PlayerBundle};
use crate::walls::{cache_wall_locations, LevelWalls, WallBundle};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResized, WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use level::{check_goal, GoalBundle};

mod grid;
mod level;
mod player;
mod walls;

const GAME_WIDTH: f32 = 380.0;
const GAME_HEIGHT: f32 = 180.0;

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
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, adjust_camera_on_resize)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell::<WallBundle>(1)
        .init_resource::<LevelWalls>()
        .add_systems(
            Update,
            (
                move_player_from_input,
                translate_grid_coords_entities,
                cache_wall_locations,
                check_goal,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, windows: Query<&mut Window>, asset_server: Res<AssetServer>) {
    let window = windows.single();
    let scale_x = window.width() / GAME_WIDTH;
    let scale_y = window.height() / GAME_HEIGHT;

    let scale = scale_x.min(scale_y);

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 1.0 / scale,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/platformer.ldtk").into(),
        ..Default::default()
    });
}

fn adjust_camera_on_resize(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    for event in resize_events.read() {
        let scale_x = event.width / GAME_WIDTH;
        let scale_y = event.height / GAME_HEIGHT;
        let scale = scale_x.min(scale_y);

        for mut projection in query.iter_mut() {
            projection.scale = 1.0 / scale;
        }
    }
}
