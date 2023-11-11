use macroquad::prelude::*;

pub fn draw_texture_rec(texture: &Texture2D, position: Vec2, rect: Rect, color: Color) {
    draw_texture_ex(
        texture,
        position.x,
        position.y,
        color,
        DrawTextureParams {
            dest_size: Some(vec2(rect.w, rect.h)),
            source: Some(rect),
            ..Default::default()
        },
    );
}
