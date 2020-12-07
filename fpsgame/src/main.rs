use bevy::{input::keyboard::KeyCode, input::mouse::MouseMotion, prelude::*, render::camera::Camera, window::WindowMode};
use player::Player;
use winit::{event_loop::EventLoop, dpi::PhysicalSize};

mod player;
mod physics;
mod math;
mod game_state;

fn main() {
    let world = physics::create_bvh_from_gltf("./assets/physics/test.glb");
    let screen_size = get_primary_physical_size();
    let mut app_builder = App::build();

    match screen_size
    {
        //if we found the screen size then use it and do borderless fullscreen
        Some(screen_size) => {
            app_builder.add_resource(WindowDescriptor {
                width: screen_size.width,
                height: screen_size.height,
                vsync: true,
                title: "Pet Tower Defense".to_string(),
                cursor_visible: false,
                cursor_locked: true,
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            });
        },
        //if we didn't find a screen size, then we do windowed
        None => {
            println!("No primary monitor detected. Using 800x600 Windowed");
            app_builder.add_resource(WindowDescriptor {
                width: 800,
                height: 600,
                vsync: true,
                title: "Pet Tower Defense".to_string(),
                cursor_visible: false,
                cursor_locked: true,
                mode: WindowMode::Windowed,
                resizable:true,
                ..Default::default()
            });
        }
    }
    app_builder
        .add_resource(Msaa { samples: 4 })
        .add_resource(world)
        .add_plugins(DefaultPlugins)
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
        .spawn((
            Player::new(4.012901, 0.3168293), 
            Transform::from_translation(Vec3::new(-3.1755996, 5.0, 2.4332705))
        )
    );
}

//TODO: Test this on not-windows
fn get_primary_physical_size() -> Option<PhysicalSize<u32>> {
    let mut size: PhysicalSize<u32> = PhysicalSize::new(1920, 1080);
    let event_loop = EventLoop::new();
    match event_loop.primary_monitor() {
        Some(monitor) => {
            size.width = monitor.size().width;
            size.height = monitor.size().height;
            return Some(size);
        },
        None => {
            return None;
        }
    };
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
    world: Res<crate::physics::World>,
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

        let ray = math::Ray::new(transform.translation + (Vec3::unit_y() * player.raycast_offset), -Vec3::unit_y(), 2.0);
        if let Some(intersection) = world.raycast(&ray) {
            let height = intersection.position.y() - player.raycast_offset;
            if height <= 0.0 {
                // in solid ground
            } else {
                // above solid ground
            }
        } else {
            // not above solid ground
            transform.translation += -Vec3::unit_y() * 0.016;
        }

        transform.translation += player_move;
    }
}

fn update_camera(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
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

        for (_camera, mut transform) in camera_query.iter_mut() {
            let camera_position = player_transform.translation + (Vec3::unit_y() * player.camera_height);
            *transform = Transform::from_translation(camera_position).looking_at(camera_position + direction, Vec3::unit_y());
        }
    }
}