use bevy::prelude::*;

pub fn get_cursor_world_position(
    windows: &Query<&Window>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    
    if let Some(cursor_pos) = window.cursor_position() {
        camera.viewport_to_world_2d(camera_transform, cursor_pos).ok()
    } else {
        None
    }
}

pub fn fade_sprite_alpha(sprite: &mut Sprite, fade_amount: f32) {
    let Srgba { red, green, blue, alpha } = sprite.color.to_srgba();
    let new_alpha = (alpha - fade_amount).max(0.0);
    sprite.color = Color::srgba(red, green, blue, new_alpha);
}

pub fn set_sprite_alpha(sprite: &mut Sprite, alpha: f32) {
    let Srgba { red, green, blue, .. } = sprite.color.to_srgba();
    sprite.color = Color::srgba(red, green, blue, alpha);
}

pub fn modify_sprite_brightness(sprite: &mut Sprite, brightness: f32) {
    let Srgba { red, green, blue, alpha } = sprite.color.to_srgba();
    sprite.color = Color::srgba(red * brightness, green * brightness, blue * brightness, alpha);
}
