use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::{ElementState, mouse::{MouseButtonInput, MouseMotion}}, prelude::*, render::camera::Camera, window::WindowMode};
use player::Player;
use command_line::*;
use util::draw_primitives::*;

mod lifetime;
mod player;
mod physics;
mod math;
mod game_state;
mod command_line;
mod util;
mod movement;

struct MainCamera;

fn main() {
    let world = physics::create_bvh_from_gltf("./assets/physics/test.glb");
    let opt: Opt = Opt::new();
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
        .add_resource(opt)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(setup_primitives.system())
        .add_startup_system(player::spawn_player.system())
        .add_system(crate::lifetime::reduce_lifetime.system())
        .add_system(update_look_direction.system())
        .add_system(player::move_player.system())
        .add_system(player::shake_when_hit_ground.system())
        .add_system(debug_player.system())
        .add_system(crate::movement::integrate_acceleration_velocity.system())
        .add_system(crate::movement::move_entities.system())
        .add_system(crate::movement::move_kinematic_entities.system())
        .add_system(update_camera.system())
        .add_system(game_state::toggle_cursor_and_exit.system())
        .add_system(player::update_trauma.system())
        .add_system(util::draw_primitives::update_primitives.system())
        .add_system(crate::lifetime::remove_entities_based_on_lifetime.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_scene(asset_server.load("physics/test.glb"))
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(MainCamera);
}

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn update_look_direction(
    mut state: Local<State>,
    time: Res<Time>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut player_query: Query<&mut Player>,
) {
    let mut mouse_delta = Vec2::default();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        mouse_delta += event.delta * time.delta_seconds * 0.1;
    }

    let mut pressed = false;
    for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
        if event.button == MouseButton::Left && event.state == ElementState::Pressed {
            pressed = true;
        }
    }

    for mut player in player_query.iter_mut() {
        player.yaw += mouse_delta.x();
        player.pitch += mouse_delta.y();

        player.pitch = player.pitch.min((0.5 * std::f32::consts::PI) - 0.01);
        player.pitch = player.pitch.max(-((0.5 * std::f32::consts::PI) - 0.01));

        player.action = pressed;
    }
}

fn debug_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<crate::physics::World>,
    player_query: Query<(&Player, &Transform)>,
) {
    for (player, transform) in player_query.iter() {
        // DEBUG raycast normal
        let pos = transform.translation + Vec3::new(0.0, player.camera_height, 0.0);
        let look = player.get_look_direction();
        let ray = crate::math::Ray::new(pos, look, std::f32::INFINITY);
        if let Some(intersection) = world.raycast(&ray) {
            crate::util::draw_primitives::draw_line_for((intersection.position, intersection.position + intersection.normal), 1);
        }

        // DEBUG spawn sphere
        if player.action {
            commands.spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.2, subdivisions: 3, })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_translation(pos),
                ..Default::default()
            }).with_bundle(crate::movement::MovementComponents {
                acceleration: crate::movement::Acceleration(look * 100.0),
                ..Default::default()
            })
            .with(crate::lifetime::Lifetime(120));
        }
    }
}

fn update_camera(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<(&MainCamera, &Camera, &mut Transform)>,
) {
    for (player, player_transform) in player_query.iter() {
        let direction = player.get_look_direction();

        for (_, _, mut transform) in camera_query.iter_mut() {
            let camera_position = player_transform.translation + (Vec3::unit_y() * player.camera_height);
            *transform = Transform::from_translation(camera_position).looking_at(camera_position + direction, Vec3::unit_y());
        }
    }
}
