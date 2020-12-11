use bevy::math::*;

use crate::math::*;

const ITERATIONS: usize = 6;

pub struct RigidBody {
    cor: f32,
    velocity: Vec3,
    mass: f32,
}

impl RigidBody {
    pub fn new(cor: f32, velocity: Vec3, mass: f32) -> Self {
        Self {
            cor,
            velocity,
            mass,
        }
    }

    pub fn inverse_mass(&self) -> f32 {
        1.0 / self.mass
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    /// Returns the coefficient of restitution (think bouncyness)
    pub fn cor(&self) -> f32 {
        self.cor
    }
}

#[derive(Debug, Clone)]
pub struct CollisionManifold {
    contact_points: Vec<Vec3>,
    normal: Vec3,
}

impl CollisionManifold {
    pub fn new(contact_points: Vec<Vec3>, normal: Vec3) -> Self {
        Self {
            contact_points,
            normal,
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_contact_points(&self) -> &[Vec3] {
        &self.contact_points
    }
}

// From a physics engine

fn apply_impulse(a: &mut RigidBody, b: &mut RigidBody, m: CollisionManifold) {
    let inv_mass_a = a.inverse_mass();
    let inv_mass_b = b.inverse_mass();
    let inv_mass_sum = inv_mass_a + inv_mass_b;

    let relative_velocity = b.velocity() - a.velocity();
    let relative_normal = m.normal();

    if relative_velocity.dot(relative_normal) > 0.0 {
        // Do nothing if bodies are moving away from each other
        return;
    }

    let e = a.cor().min(b.cor());
    let numerator = -(1.0 + e) * relative_velocity.dot(relative_normal);
    let mut j = numerator / inv_mass_sum;
    if m.get_contact_points().len() > 0 && !j.approximately(0.0) {
        j /= m.get_contact_points().len() as f32;
    }

    let impulse = relative_normal * j;
    a.set_velocity(a.velocity() + (impulse * inv_mass_a * -1.0));
    b.set_velocity(b.velocity() + (impulse * inv_mass_b * 1.0));
}

// Simplified to one body being static

pub fn apply_impulse_b_is_static(a: &mut RigidBody, b: &mut RigidBody, m: CollisionManifold) {
    let relative_velocity = a.velocity();
    let relative_normal = m.normal();

    if relative_velocity.dot(relative_normal) > 0.0 {
        // Do nothing if bodies are moving away from each other
        return;
    }

    let e = a.cor();
    let mut j = -(1.0 + e) * relative_velocity.dot(relative_normal);
    if m.get_contact_points().len() > 0 && !j.approximately(0.0) {
        j /= m.get_contact_points().len() as f32;
    }

    let impulse = relative_normal * j;
    a.set_velocity(a.velocity() + (impulse * -1.0));
}

// Refactored to what we need

pub fn resolve_collisions(mut velocity: Vec3, cor: f32, contact_normals: &[Vec3]) -> Vec3 {

    let total_contacts = contact_normals.len() as f32;
    for normal in contact_normals {
        velocity = apply_collision_impulses(velocity, cor, *normal, total_contacts);
        dbg!(velocity);
    }

    dbg!(velocity);

    velocity
}

pub fn apply_collision_impulses(velocity: Vec3, cor: f32, contact_normal: Vec3, total_contacts: f32) -> Vec3 {
    if velocity.dot(contact_normal) > 0.0 {
        // Do nothing if bodies are moving away from each other
        return velocity;
    }

    let j = (-(1.0 + cor) * velocity.dot(contact_normal)) / total_contacts;

    let impulse = contact_normal * j;
    velocity + (impulse * -1.0)
}
