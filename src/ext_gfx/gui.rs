use macroquad::{prelude::*, ui::widgets::Button};

pub struct GUITextures {
    arrow_up: Texture2D,
    arrow_down: Texture2D,
}

pub struct GUI {
    textures: GUITextures,
}

impl GUI {
    pub fn button(&self, bounds: Rect, text: &str) -> bool {
        let mut clicked = false;

        let mut bg = color_u8!(0xff, 0xff, 0xff, 0xff);
        let mut c3 = color_u8!(0xdf, 0xdf, 0xdf, 0xff);
        let mut c2 = color_u8!(0xba, 0xba, 0xba, 0xff);
        let mut c1 = color_u8!(0x9a, 0x9a, 0x9a, 0xff);
        let mut c0 = color_u8!(0x45, 0x45, 0x45, 0xff);

        if bounds.contains(Vec2::from(mouse_position())) {
            bg = color_u8!(0xdf, 0xdf, 0xdf, 0xff);
            c3 = color_u8!(0xba, 0xba, 0xba, 0xff);
            c2 = color_u8!(0x9a, 0x9a, 0x9a, 0xff);
            c1 = color_u8!(0x75, 0x75, 0x75, 0xff);
            c0 = color_u8!(0x45, 0x45, 0x45, 0xff);

            clicked = is_mouse_button_pressed(MouseButton::Left);
        };

        // Draw base
        draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, c0);
        draw_rectangle(
            bounds.x + 1.0,
            bounds.y + 1.0,
            bounds.w - 2.0,
            bounds.h - 2.0,
            c1,
        );
        draw_rectangle(
            bounds.x + 2.0,
            bounds.y + 2.0,
            bounds.w - 4.0,
            bounds.h - 4.0,
            c2,
        );
        draw_rectangle(
            bounds.x + 3.0,
            bounds.y + 3.0,
            bounds.w - 6.0,
            bounds.h - 6.0,
            c3,
        );
        draw_rectangle(
            bounds.x + 4.0,
            bounds.y + 4.0,
            bounds.w - 8.0,
            bounds.h - 8.0,
            bg,
        );

        // Draw text
        let text_size = measure_text(text, None, 32, 1.0);

        draw_text(
            text,
            bounds.x + (bounds.w - text_size.width) / 2.0,
            bounds.y + bounds.h - text_size.height / 2.0,
            32.0,
            BLACK,
        );

        clicked
    }

    pub fn spinner(x: f32, y: f32, value: i32, min: i32, max: i32) -> i32 {
        let v = value;

        v
    }

    pub async fn new() -> GUI {
        GUI {
            textures: GUITextures {
                arrow_up: load_texture("./resources/textures/ui_arrow_up.png")
                    .await
                    .unwrap(),
                arrow_down: load_texture("./resources/textures/ui_arrow_down.png")
                    .await
                    .unwrap(),
            },
        }
    }
}
