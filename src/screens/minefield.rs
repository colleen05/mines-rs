use crate::*;

impl Game {
    pub fn do_minefield(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::InMenu;
        }
        /* Draw panels */
        {
            draw_rectangle(
                // Solid BG - Top
                0.0,
                0.0,
                screen_width(),
                120.0,
                color_u8!(0xff, 0xff, 0xff, 0xff),
            );
            draw_rectangle_lines(
                // Border - Top, Outside
                0.0,
                0.0,
                screen_width(),
                120.0,
                2.0,
                color_u8!(0x55, 0x55, 0x55, 0xff),
            );
            draw_rectangle_lines(
                // Border - Top, Inside
                1.0,
                1.0,
                screen_width() - 2.0,
                118.0,
                2.0,
                color_u8!(0xaa, 0xaa, 0xaa, 0xff),
            );

            draw_rectangle(
                // Solid BG - Bottom
                0.0,
                120.0,
                800.0,
                480.0,
                color_u8!(0xaa, 0xaa, 0xaa, 0xff),
            );
            draw_rectangle_lines(
                // Border - Bottom, Outside
                0.0,
                120.0,
                800.0,
                480.0,
                2.0,
                color_u8!(0x75, 0x75, 0x75, 0xff),
            );
            draw_rectangle_lines(
                // Border - Bottom, Inside
                1.0,
                121.0,
                798.0,
                478.0,
                2.0,
                color_u8!(0x8a, 0x8a, 0x8a, 0xff),
            );
        }

        /* Draw status bar */
        {
            // Status face
            draw_texture_rec(
                &self.assets.textures.status_bar,
                Vec2 {
                    x: screen_width() / 2.0 - 126.0,
                    y: 12.0,
                },
                self.status_face.get_texture_rect(),
                WHITE,
            );

            // Mines counter - Icon
            draw_texture_rec(
                &self.assets.textures.status_bar,
                Vec2 { x: 32.0, y: 36.0 },
                StatusSegment::Mine.get_texture_rect(),
                WHITE,
            );

            // Mines counter - zeros
            for i in 0..3 {
                draw_texture_rec(
                    &self.assets.textures.status_bar,
                    Vec2 {
                        x: 64.0 + 32.0 * (i as f32),
                        y: 36.0,
                    },
                    StatusSegment::Blank.get_texture_rect(),
                    WHITE,
                )
            }
        }
    }
}
