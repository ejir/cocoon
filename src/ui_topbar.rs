use bevy::prelude::*;

use crate::bomb::spawn_bomb_from_ui;
use crate::combustion::spawn_fire_from_ui;
use crate::drag::DragState;
use crate::iron_block::spawn_iron_block_from_ui;
use crate::ragdoll::spawn_ragdoll_from_ui;
use crate::utils::get_cursor_world_position;
use crate::wooden_box::spawn_wooden_box_from_ui;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectType {
    Ragdoll,
    Bomb,
    WoodenBox,
    IronBlock,
    Fire,
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
        });
}

fn create_object_button(
    parent: &mut ChildBuilder,
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
    flammable_query: Query<(Entity, &Transform, &crate::components::Flammable), Without<crate::components::OnFire>>,
) {
    if mouse_button.just_released(MouseButton::Left) && drag_state.dragging_entity.is_none() {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            match selected_object.object_type {
                ObjectType::Ragdoll => spawn_ragdoll_from_ui(&mut commands, world_pos),
                ObjectType::Bomb => spawn_bomb_from_ui(&mut commands, world_pos),
                ObjectType::WoodenBox => spawn_wooden_box_from_ui(&mut commands, world_pos),
                ObjectType::IronBlock => spawn_iron_block_from_ui(&mut commands, world_pos),
                ObjectType::Fire => spawn_fire_from_ui(&mut commands, world_pos, &flammable_query),
            }
        }
    }
}
