use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::get_cursor_world_position;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintType {
    Fixed,
    Hinge,
}

#[derive(Resource)]
pub struct SelectionState {
    pub first_selected: Option<Entity>,
    pub second_selected: Option<Entity>,
    pub constraint_type: ConstraintType,
    pub is_enabled: bool,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            first_selected: None,
            second_selected: None,
            constraint_type: ConstraintType::Fixed,
            is_enabled: false,
        }
    }
}

#[derive(Component)]
pub struct SelectionIndicator {
    pub target_entity: Entity,
    pub is_first: bool,
}

#[derive(Component)]
pub struct Connectable;

#[derive(Component)]
pub struct UserCreatedJoint;

pub fn handle_object_selection(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
    drag_state: Res<crate::drag::DragState>,
) {
    if !selection_state.is_enabled {
        return;
    }

    // Don't handle selection if dragging
    if drag_state.dragging_entity.is_some() {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            let mut closest_entity = None;
            let mut closest_distance = f32::INFINITY;

            for (entity, transform) in connectable_query.iter() {
                let object_pos = transform.translation.truncate();
                let distance = object_pos.distance(world_pos);

                let max_radius = 50.0;

                if distance < max_radius && distance < closest_distance {
                    closest_distance = distance;
                    closest_entity = Some(entity);
                }
            }

            if let Some(entity) = closest_entity {
                if selection_state.first_selected.is_none() {
                    selection_state.first_selected = Some(entity);
                    spawn_selection_indicator(&mut commands, entity, true);
                } else if selection_state.first_selected == Some(entity) {
                    clear_selection(&mut commands, &mut selection_state, &indicator_query);
                } else if selection_state.second_selected.is_none() {
                    if selection_state.first_selected != Some(entity) {
                        selection_state.second_selected = Some(entity);
                        spawn_selection_indicator(&mut commands, entity, false);
                    }
                } else {
                    clear_selection(&mut commands, &mut selection_state, &indicator_query);
                    selection_state.first_selected = Some(entity);
                    spawn_selection_indicator(&mut commands, entity, true);
                }
            }
        }
    }
}

fn spawn_selection_indicator(commands: &mut Commands, target_entity: Entity, is_first: bool) {
    let color = if is_first {
        Color::srgba(0.0, 1.0, 0.0, 0.6)
    } else {
        Color::srgba(0.0, 0.5, 1.0, 0.6)
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(60.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        SelectionIndicator {
            target_entity,
            is_first,
        },
    ));
}

fn clear_selection(
    commands: &mut Commands,
    selection_state: &mut SelectionState,
    indicator_query: &Query<Entity, With<SelectionIndicator>>,
) {
    selection_state.first_selected = None;
    selection_state.second_selected = None;

    for entity in indicator_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_selection_indicators(
    mut indicator_query: Query<(&mut Transform, &SelectionIndicator)>,
    transform_query: Query<&Transform, Without<SelectionIndicator>>,
) {
    for (mut indicator_transform, indicator) in indicator_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(indicator.target_entity) {
            indicator_transform.translation = target_transform.translation + Vec3::new(0.0, 0.0, 1.0);
            indicator_transform.rotation = target_transform.rotation;
        }
    }
}

pub fn create_constraint_system(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
    transform_query: Query<&Transform>,
) {
    if !selection_state.is_enabled {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyC) || keyboard.just_pressed(KeyCode::Enter) {
        if let (Some(first), Some(second)) = (selection_state.first_selected, selection_state.second_selected) {
            if let (Ok(first_transform), Ok(second_transform)) = (
                transform_query.get(first),
                transform_query.get(second),
            ) {
                let first_pos = first_transform.translation.truncate();
                let second_pos = second_transform.translation.truncate();

                let midpoint = (first_pos + second_pos) / 2.0;
                let anchor1 = midpoint - first_pos;
                let anchor2 = midpoint - second_pos;

                match selection_state.constraint_type {
                    ConstraintType::Fixed => {
                        let joint = FixedJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                        ));
                    }
                    ConstraintType::Hinge => {
                        let joint = RevoluteJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                        ));
                    }
                }

                for entity in indicator_query.iter() {
                    commands.entity(entity).despawn();
                }

                selection_state.first_selected = None;
                selection_state.second_selected = None;
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        clear_selection(&mut commands, &mut selection_state, &indicator_query);
    }
}

pub fn handle_deleted_selections(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    query: Query<Entity, With<Connectable>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
) {
    let mut should_clear = false;

    if let Some(first) = selection_state.first_selected {
        if query.get(first).is_err() {
            should_clear = true;
        }
    }

    if let Some(second) = selection_state.second_selected {
        if query.get(second).is_err() {
            should_clear = true;
        }
    }

    if should_clear {
        clear_selection(&mut commands, &mut selection_state, &indicator_query);
    }
}
