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

    pub fn spinner(&self, x: f32, y: f32, mut value: i32, min: i32, max: i32) -> i32 {
        // Define bounds
        let bounds = Rect::new(x, y, 68.0, 32.0);

        // Draw base
        draw_rectangle(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            color_u8!(0x45, 0x45, 0x45, 0xff),
        );

        draw_rectangle(
            bounds.x + 1.0,
            bounds.y + 1.0,
            bounds.w - 21.0 - 2.0,
            bounds.h - 2.0,
            color_u8!(0x9a, 0x9a, 0x9a, 0xff),
        );

        draw_rectangle(
            bounds.x + 2.0,
            bounds.y + 2.0,
            bounds.w - 21.0 - 4.0,
            bounds.h - 4.0,
            color_u8!(0xdf, 0xdf, 0xdf, 0xff),
        );

        let text_size = measure_text(value.to_string().as_str(), None, 32, 1.0);

        draw_text(
            value.to_string().as_str(),
            bounds.x + bounds.w - text_size.width - 24.0,
            bounds.y + 24.0,
            32.0,
            BLACK,
        );

        // Buttons
        if self.button(
            Rect::new(bounds.x + bounds.w - 22.0, bounds.y, 22.0, bounds.h / 2.0),
            "",
        ) {
            value += 1;
        }

        if self.button(
            Rect::new(
                bounds.x + bounds.w - 22.0,
                bounds.y + 16.0,
                22.0,
                bounds.h / 2.0,
            ),
            "",
        ) {
            value -= 1;
        }

        draw_texture(
            &self.textures.arrow_up,
            bounds.x + bounds.w - 22.0,
            bounds.y,
            color_u8!(0x45, 0x45, 0x45, 0xff),
        );

        draw_texture(
            &self.textures.arrow_down,
            bounds.x + bounds.w - 22.0,
            bounds.y + 16.0,
            color_u8!(0x45, 0x45, 0x45, 0xff),
        );

        // Scrollilng
        if bounds.contains(Vec2::from(mouse_position())) {
            value += mouse_wheel().1 as i32;
        }

        clamp(value, min, max)
    }

    pub async fn new(theme_name: &str) -> GUI {
        let resource_path = format!("./resources/themes/{theme_name}");

        GUI {
            textures: GUITextures {
                arrow_up: load_texture(
                    format!("{resource_path}/textures/ui_arrow_up.png").as_str(),
                )
                .await
                .unwrap(),
                arrow_down: load_texture(
                    format!("{resource_path}/textures/ui_arrow_down.png").as_str(),
                )
                .await
                .unwrap(),
            },
        }
    }
}
