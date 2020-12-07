use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::keyboard::KeyCode, input::mouse::MouseMotion, prelude::*, render::camera::Camera, window::WindowMode};
use player::Player;

mod player;
mod physics;
mod math;
mod game_state;

struct MainCamera;

fn main() {
    let world = physics::create_bvh_from_gltf("./assets/physics/test.glb");

    App::build()
        .add_resource(WindowDescriptor {
            width: 1920,
            height: 1080,
            vsync: true,
            title: "Pet Tower Defense".to_string(),
            cursor_visible: false,
            cursor_locked: true,
            mode: WindowMode::Windowed,
            resizable: true,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_resource(world)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(update_look_direction.system())
        .add_system(move_player.system())
        .add_system(update_camera.system())
        .add_system(game_state::toggle_cursor_and_exit.system())
        .add_system(text_update_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_scene(asset_server.load("physics/test.glb"))
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
        .with(MainCamera)
        .spawn((Player::new(4.012901, 0.3168293), Transform::from_translation(Vec3::new(-3.1755996, 5.0, 2.4332705))));

    commands
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with(FpsText);
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

        player.pitch = player.pitch.min((0.5 * std::f32::consts::PI) - 0.01);
        player.pitch = player.pitch.max(-((0.5 * std::f32::consts::PI) - 0.01));
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    world: Res<crate::physics::World>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
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

    for (mut player, mut transform) in player_query.iter_mut() {
        let mut is_grounded = false;
        let mut on_slope = false;
        let sin = player.yaw.sin();
        let cos = player.yaw.cos();

        player_move *= 0.016 * 3.0;

        let player_move = Vec3::new(
            player_move.x() * cos - player_move.z() * sin,
            player_move.y() * 12.0,
            player_move.z() * cos + player_move.x() * sin,
        );

        let ray = math::Ray::new(transform.translation + (Vec3::unit_y() * player.raycast_offset), -Vec3::unit_y(), 2.0);
        if let Some(intersection) = world.raycast(&ray) {

            let height = intersection.t - player.raycast_offset;
            if height < 0.01 {
                transform.translation += Vec3::unit_y() * (-height - 0.01);
                is_grounded = true;
            }

            if intersection.normal.dot(Vec3::unit_y()) <= 0.999 {
                on_slope = true;
            }
            
        } else {
            // not above solid ground
            is_grounded = false;
        }

        if !is_grounded {
            transform.translation += Vec3::unit_y() * 0.016 * -9.81;
        }

        transform.translation += player_move;
        
        player.was_grounded = player.grounded;
        player.grounded = is_grounded;
        player.was_on_slope = player.on_slope;
        player.on_slope = on_slope;
        if player.grounded {
            player.frames_since_grounded = 0;
        } else {
            player.frames_since_grounded += 1;
        }
    }
}

fn update_camera(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<(&MainCamera, &Camera, &mut Transform)>,
) {
    for (player, player_transform) in player_query.iter() {
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let direction = Vec3::new(
            direction.x(),
            direction.y() * player.pitch.cos() - direction.z() * player.pitch.sin(),
            direction.z() * player.pitch.cos() - direction.y() * player.pitch.sin(),
        );

        let direction = Vec3::new(
            direction.x() * player.yaw.cos() - direction.z() * player.yaw.sin(),
            direction.y(),
            direction.z() * player.yaw.cos() - direction.x() * player.yaw.sin(),
        );

        let direction = direction.normalize();

        for (_, _, mut transform) in camera_query.iter_mut() {
            let camera_position = player_transform.translation + (Vec3::unit_y() * player.camera_height);
            *transform = Transform::from_translation(camera_position).looking_at(camera_position + direction, Vec3::unit_y());
        }
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;
fn text_update_system(diagnostics: Res<bevy::diagnostic::Diagnostics>, mut query: Query<(&mut Text, &FpsText)>, player_query: Query<&Player>) {

    let player = player_query.iter().next().unwrap();

    for (mut text, _tag) in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}, WG: {}, G: {}, WS: {}, S: {}, {}", average, player.was_grounded, player.grounded, player.was_on_slope, player.on_slope, player.frames_since_grounded);
            }
        }
    }
}
