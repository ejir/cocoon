use bevy::prelude::*;

pub fn find_closest_entity<'a, T>(
    query: impl Iterator<Item = (Entity, &'a Transform, T)>,
    target_pos: Vec2,
    max_distance: f32,
) -> Option<Entity> {
    let mut closest_entity = None;
    let mut closest_distance = f32::INFINITY;

    for (entity, transform, _) in query {
        let pos = transform.translation.truncate();
        let distance = pos.distance(target_pos);
        if distance < max_distance && distance < closest_distance {
            closest_distance = distance;
            closest_entity = Some(entity);
        }
    }

    closest_entity
}
