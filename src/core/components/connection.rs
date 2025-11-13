//! Connection system components

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionKind {
    Fixed,
    Hinge,
}

#[derive(Component)]
pub struct Connection {
    pub a: Entity,
    pub b: Entity,
    pub anchor_a: Vec2,
    pub anchor_b: Vec2,
    pub kind: ConnectionKind,
    pub break_force: f32,
    pub current_force: f32,
}
