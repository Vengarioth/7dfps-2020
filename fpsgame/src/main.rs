use bevy::{input::keyboard::KeyCode, input::mouse::MouseMotion, prelude::*, render::camera::Camera, window::WindowMode};
use bevy_rapier3d::{physics::RapierPhysicsPlugin, render::RapierRenderPlugin, rapier::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder}};

use player::Player;

mod player;
mod physics;
mod math;
mod game_state;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 1920,
            height: 1080,
            vsync: true,
            title: "Pet Tower Defense".to_string(),
            cursor_visible: false,
            cursor_locked: true,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .add_system(update_look_direction.system())
        .add_system(move_player.system())
        .add_system(update_camera.system())
        .add_system(game_state::toggle_cursor_and_exit.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .spawn((Player {
            yaw: 0.0,
            pitch: 0.0,
        }, Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))));

    let rigid_body1 = RigidBodyBuilder::new_static();
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0, 10.0);
    commands.spawn((rigid_body1, collider1));

    let rigid_body2 = RigidBodyBuilder::new_dynamic().translation(0.0, 3.0, 0.0);
    let collider2 = ColliderBuilder::ball(0.5);
    commands.spawn((rigid_body2, collider2));
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn update_look_direction(
    mut state: Local<State>,
    time: Res<Time>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut player_query: Query<&mut Player>,
) {
    let mut mouse_delta = Vec2::default();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        mouse_delta += event.delta * time.delta_seconds * 0.1;
    }

    for mut player in player_query.iter_mut() {
        player.yaw += mouse_delta.x();
        player.pitch += mouse_delta.y();
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let mut player_move = Vec3::default();
    if keyboard_input.pressed(KeyCode::W) {
         player_move += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
         player_move += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
         player_move += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
         player_move += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Space) {
         player_move += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::LShift) {
         player_move += Vec3::new(0.0, -1.0, 0.0);
    }

    for (player, mut transform) in player_query.iter_mut() {
        let sin = player.yaw.sin();
        let cos = player.yaw.cos();

        let player_move = Vec3::new(
            player_move.x() * cos - player_move.z() * sin,
            player_move.y(),
            player_move.z() * cos + player_move.x() * sin,
        );

        transform.translation += player_move;
    }
}

fn update_camera(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    for (player, player_transform) in player_query.iter() {
        let rotation = Quat::from_rotation_ypr(player.yaw, player.pitch, 0.0);
        let direction = Vec3::unit_z();
        let direction = (rotation * direction).normalize();

        for (_camera, mut transform) in camera_query.iter_mut() {
            *transform = Transform::from_translation(player_transform.translation)
            .looking_at(player_transform.translation + direction, Vec3::unit_y());
        }
    }
}