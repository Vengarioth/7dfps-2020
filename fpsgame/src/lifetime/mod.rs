use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Lifetime(pub u32);

pub fn reduce_lifetime(mut entities: Query<&mut Lifetime>) {
    for mut lifetime in entities.iter_mut() {
        if lifetime.0 > 0 {
            lifetime.0 -= 1;
        }
    }
}

pub fn remove_entities_based_on_lifetime(mut commands: Commands, entities: Query<(Entity, &Lifetime)>) {
    for (entity, lifetime) in entities.iter() {
        if lifetime.0 == 0 {
            commands.despawn(entity);
        }
    }
}
