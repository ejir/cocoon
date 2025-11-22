use bevy::prelude::*;

use crate::core::components::{Flammable, OnFire};
use crate::core::utils::get_cursor_world_position;
use crate::entities::ragdoll::spawn_ragdoll_from_ui;
use crate::entities::weapons::spawn_bomb_from_ui;
use crate::systems::damage::connection::{ConstraintType, SelectionState, ConnectionMaterial};
use crate::systems::effects::spawn_fire_from_ui;
use crate::systems::input::drag::DragState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectType {
    Ragdoll,
    Bomb,
    WoodenBox,
    IronBlock,
    Fire,
    FixedConstraint,
    HingeConstraint,
    // Material selection for connections
    MaterialWood,
    MaterialMetal,
    MaterialRope,
    MaterialPlastic,
}

#[derive(Resource)]
pub struct SelectedObject {
    pub object_type: ObjectType,
}

impl Default for SelectedObject {
    fn default() -> Self {
        Self {
            object_type: ObjectType::Ragdoll,
        }
    }
}

#[derive(Component)]
pub struct ObjectButton {
    pub object_type: ObjectType,
}

#[derive(Component)]
pub struct TopBarUI;

#[derive(Component)]
pub struct MaterialButton;

pub fn setup_ui_topbar(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                column_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.9)),
            TopBarUI,
        ))
        .with_children(|parent| {
            create_object_button(parent, ObjectType::Ragdoll, "Ragdoll (R)", true);
            create_object_button(parent, ObjectType::Bomb, "Bomb (B)", false);
            create_object_button(parent, ObjectType::WoodenBox, "Box (W)", false);
            create_object_button(parent, ObjectType::IronBlock, "Iron (I)", false);
            create_object_button(parent, ObjectType::Fire, "Fire (F)", false);
            create_object_button(parent, ObjectType::FixedConstraint, "Fixed (X)", false);
            create_object_button(parent, ObjectType::HingeConstraint, "Hinge (H)", false);
        });
}

fn create_object_button(
    parent: &mut ChildSpawner,
    object_type: ObjectType,
    label: &str,
    is_selected: bool,
) {
    let bg_color = if is_selected {
        Color::srgb(0.3, 0.5, 0.7)
    } else {
        Color::srgb(0.25, 0.25, 0.25)
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            ObjectButton { object_type },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

pub fn handle_button_clicks(
    mut selected_object: ResMut<SelectedObject>,
    mut queries: ParamSet<(
        Query<
            (&Interaction, &ObjectButton, &mut BackgroundColor),
            Changed<Interaction>,
        >,
        Query<(&ObjectButton, &mut BackgroundColor)>,
    )>,
) {
    let mut pressed_button: Option<ObjectType> = None;

    for (interaction, button, mut bg_color) in queries.p0().iter_mut() {
        if *interaction == Interaction::Pressed {
            pressed_button = Some(button.object_type);
            selected_object.object_type = button.object_type;
        } else if *interaction == Interaction::Hovered {
            if selected_object.object_type != button.object_type {
                *bg_color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
            }
        } else if selected_object.object_type != button.object_type {
            *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
        }
    }

    if let Some(pressed_type) = pressed_button {
        for (button, mut bg_color) in queries.p1().iter_mut() {
            if button.object_type == pressed_type {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.5, 0.7));
            } else {
                *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
        }
    }
}

pub fn spawn_selected_object_on_click(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_object: Res<SelectedObject>,
    drag_state: Res<DragState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    flammable_query: Query<(Entity, &Transform, &Flammable), Without<OnFire>>,
) {
    if mouse_button.just_released(MouseButton::Left) && drag_state.dragging_entity.is_none() {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            match selected_object.object_type {
                ObjectType::Ragdoll => spawn_ragdoll_from_ui(&mut commands, world_pos),
                ObjectType::Bomb => spawn_bomb_from_ui(&mut commands, world_pos),
                // WoodenBox and IronBlock now use drag-to-create, so skip here
                ObjectType::WoodenBox => {},
                ObjectType::IronBlock => {},
                ObjectType::Fire => spawn_fire_from_ui(&mut commands, world_pos, &flammable_query),
                // FixedConstraint and HingeConstraint are handled by the connection system
                ObjectType::FixedConstraint => {},
                ObjectType::HingeConstraint => {},
                // Material selection buttons don't spawn objects
                ObjectType::MaterialWood => {},
                ObjectType::MaterialMetal => {},
                ObjectType::MaterialRope => {},
                ObjectType::MaterialPlastic => {},
            }
        }
    }
}

pub fn sync_selection_with_connection_system(
    mut commands: Commands,
    selected_object: Res<SelectedObject>,
    mut selection_state: ResMut<SelectionState>,
    material_button_query: Query<Entity, With<MaterialButton>>,
) {
    if selected_object.is_changed() {
        let was_enabled = selection_state.is_enabled;
        
        match selected_object.object_type {
            ObjectType::FixedConstraint => {
                selection_state.is_enabled = true;
                selection_state.constraint_type = ConstraintType::Fixed;
            }
            ObjectType::HingeConstraint => {
                selection_state.is_enabled = true;
                selection_state.constraint_type = ConstraintType::Hinge;
            }
            // Material selection - changes material but keeps connection mode active
            ObjectType::MaterialWood => {
                selection_state.material = ConnectionMaterial::Wood;
                // Don't change is_enabled - keep current connection mode active
                return;
            }
            ObjectType::MaterialMetal => {
                selection_state.material = ConnectionMaterial::Metal;
                return;
            }
            ObjectType::MaterialRope => {
                selection_state.material = ConnectionMaterial::Rope;
                return;
            }
            ObjectType::MaterialPlastic => {
                selection_state.material = ConnectionMaterial::Plastic;
                return;
            }
            _ => {
                selection_state.is_enabled = false;
            }
        }
        
        // Show/hide material buttons based on whether connection mode is active
        if !was_enabled && selection_state.is_enabled {
            spawn_material_buttons(&mut commands, selection_state.material);
        } else if was_enabled && !selection_state.is_enabled {
            for entity in material_button_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn spawn_material_buttons(commands: &mut Commands, current_material: ConnectionMaterial) {
    // Spawn a panel below the top bar with material selection buttons
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            width: Val::Px(520.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
        MaterialButton,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Material:"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
        
        create_material_button(parent, ObjectType::MaterialWood, "Wood", current_material == ConnectionMaterial::Wood);
        create_material_button(parent, ObjectType::MaterialMetal, "Metal", current_material == ConnectionMaterial::Metal);
        create_material_button(parent, ObjectType::MaterialRope, "Rope", current_material == ConnectionMaterial::Rope);
        create_material_button(parent, ObjectType::MaterialPlastic, "Plastic", current_material == ConnectionMaterial::Plastic);
    });
}

fn create_material_button(
    parent: &mut ChildSpawner,
    object_type: ObjectType,
    label: &str,
    is_selected: bool,
) {
    let bg_color = if is_selected {
        Color::srgb(0.3, 0.5, 0.7)
    } else {
        Color::srgb(0.25, 0.25, 0.25)
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            ObjectButton { object_type },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
