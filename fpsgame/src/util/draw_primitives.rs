use std::{sync::Mutex, mem};
use once_cell::sync::Lazy;
use bevy::{math::Vec3, prelude::*};

// ok so whats going on here: we have a global list and 2 systems.
// the global list "SCHEDULED_PRIMITIVES" is where the draw_line functions 
// push their primitives.
// The first system "setup_primitives" is setting up the shared state
// (which is draw_primitives::State). 
// The second system "update_primitives" takes the primitives out of
// the global "SCHEDULED_PRIMITIVES" list and instantiates new primitives as
// entities which are drawn. It also decreases the "frames_left" counter of
// all entities and despawns them if the counter reaches 0

///draw a worldspace line for a single frame
pub fn draw_line(from_to: (Vec3, Vec3)) -> (Vec3, Vec3) {
    draw_line_for(from_to, 1)
}

///draw a worldspace line for a certain amount of frames before it will disappear
pub fn draw_line_for(from_to: (Vec3, Vec3), for_frames: u32) -> (Vec3, Vec3) {
    let mut scheduled = SCHEDULED_PRIMITIVES.lock().unwrap();
    scheduled.push(Primitive {
        shape: Shape::Line(from_to), 
        frames_left: for_frames,
    });
    from_to
}

///a list of primitives which are about to be spawned as entities in the next invocation of `update_primitives`
static SCHEDULED_PRIMITIVES: Lazy<Mutex<Vec<Primitive>>> = Lazy::new(|| Mutex::new(Vec::new()));

///locks the mutex and swaps out all the primitives we have to draw
fn swap_sheduled_primitives(mut target: &mut Vec<Primitive>) {
    assert!(target.is_empty()); //we expect the target to already be drained
    {   //take all the new primitives out of the mutex
        let mut guard = SCHEDULED_PRIMITIVES.lock().unwrap();
        mem::swap(&mut *guard, &mut target);
    }
}

///transform required to make a line out of a unit cube, center at 0, 0, 0
fn line_transform_for_unit_cube((mut a, mut b): (Vec3, Vec3)) -> Transform {
    let delta = b - a;

    //unit cube has interval [-.5, .5]^3. quick fix for it is to shift the line by half
    a += delta * 0.5;
    b += delta * 0.5;

    let length = delta.length();
    let thickness = 0.1; //we could pull this out later as an argument in draw_lines
    let basis0 = delta.normalize();

    //choose helper vec3 that is not too close to basis0's axis
    let helper = match basis0.dot(Vec3::unit_y()).abs() < 0.5 {
        true  => Vec3::unit_y(),
        false => Vec3::unit_x(),
    };

    //make 2 additional basis vectors, so that all 3 are perpendicular
    let basis1 = basis0.cross(helper).normalize();
    let basis2 = basis0.cross(basis1);

    //scale basis vectors by factors to stretch the cube to look like a line
    let mat4 = Mat4::from_cols(
        basis0.extend(0.0) * length,
        basis1.extend(0.0) * thickness,
        basis2.extend(0.0) * thickness,
        a.extend(1.0), //translation
    );

    Transform::from_matrix(mat4)
}

///shapes which use world space coordinates
#[derive(Debug)]
enum Shape {
    Line((Vec3, Vec3)), 
}

///Drawable primitive ecs component
#[derive(Debug)]
pub struct Primitive {
    shape: Shape,
    frames_left: u32, //how many frames the primitive is still visible
}

///Persistent state ecs component
#[derive(Default)]
pub struct State {
    new_primitives: Vec::<Primitive>, //reused buffer for swapping with SCHEDULED_PRIMITIVES
    line_cube_mesh: Handle<Mesh>,
    line_material: Handle<StandardMaterial>,
}

/// only purpose of this system is to init the shared State, 
/// the real fun happens at update_primitives
pub fn setup_primitives(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    let state = State {
        line_cube_mesh: meshes.add(Mesh::from(shape::Cube{size: 0.5})),
        line_material: materials.add(Color::rgb(0.0, 0.0, 1000.0).into()), //perfectly normal rgb
        ..Default::default()
    };
    commands.spawn((state, ));
}

/// takes the primitives out of SCHEDULED_PRIMITIVES and turns them into entities
/// updates entities by decreasing their "frames_left" value. Once it hits 0, the entity is despawned
pub fn update_primitives(
    mut commands: Commands,
    mut state: Query<&mut State>,
    mut primitives: Query<(Entity, &mut Primitive)>) {

    let mut state = state.iter_mut().next()
    .expect("draw_primitives::State was not initialized, did you forget to register the `setup_primitives` system?");

    let line_cube_mesh = state.line_cube_mesh.clone(); //because partial borrows...
    let line_material  = state.line_material.clone();

    //fetch the scheduled primitives from the static mutex
    swap_sheduled_primitives(&mut state.new_primitives);

    //spawn a new entity for every primitive
    for primitive in state.new_primitives.drain(..) {
        let tf = match primitive.shape {
            Shape::Line(from_to) => {
                //Transform::from_translation(from_to.0);
                line_transform_for_unit_cube(from_to)
            }
        };
        commands.spawn(PbrComponents{
            //meshes.add is probably bad here, need to figure out how to reuse meshes
            mesh:    line_cube_mesh.clone(),
            material: line_material.clone(),
            transform: tf,
            ..Default::default()
        })
        .with(primitive);
    }
    assert!(state.new_primitives.is_empty());

    // update existing primitives, despawn them if their time has come
    for (entity, mut primitive) in primitives.iter_mut() {
        primitive.frames_left -= 1;

        if primitive.frames_left <= 0 {
            commands.despawn(entity);
        }
    }
}
