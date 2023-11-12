use crate::*;

impl Game {
    fn draw_bg(&mut self) {
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
            screen_width(),
            screen_height() - 120.0,
            color_u8!(0xaa, 0xaa, 0xaa, 0xff),
        );
        draw_rectangle_lines(
            // Border - Bottom, Outside
            0.0,
            120.0,
            screen_width(),
            screen_height() - 120.0,
            2.0,
            color_u8!(0x75, 0x75, 0x75, 0xff),
        );
        draw_rectangle_lines(
            // Border - Bottom, Inside
            1.0,
            121.0,
            screen_width() - 2.0,
            screen_height() - 122.0,
            2.0,
            color_u8!(0x8a, 0x8a, 0x8a, 0xff),
        );
    }

    fn draw_7seg(&self, position: Vec2, segments: usize, num: i32) {
        let text = num.to_string();

        if text.len() > segments {
            return;
        };

        for i in 0..segments {
            let mut texture_rec = StatusSegment::Blank.get_texture_rect();

            if i >= segments - text.len() {
                let chr = text.chars().nth(i - (segments - text.len())).unwrap();

                if chr.is_ascii_digit() {
                    texture_rec = StatusSegment::try_from(chr.to_digit(10).unwrap() as i32)
                        .unwrap()
                        .get_texture_rect();
                } else {
                    texture_rec = StatusSegment::Mine.get_texture_rect();
                }
            }

            draw_texture_rec(
                &self.assets.textures.status_bar,
                Vec2 {
                    x: position.x + (i as f32) * 32.0,
                    y: position.y,
                },
                texture_rec,
                WHITE,
            )
        }
    }

    fn draw_status_bar(&mut self) {
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

        // Mines counter
        draw_texture_rec(
            &self.assets.textures.status_bar,
            Vec2 { x: 32.0, y: 36.0 },
            StatusSegment::Mine.get_texture_rect(),
            WHITE,
        );

        self.draw_7seg(
            Vec2 { x: 64.0, y: 36.0 },
            3,
            self.field_mines - self.placed_flags,
        );

        // Timer
        self.draw_7seg(
            Vec2 {
                x: screen_width() - 160.0,
                y: 36.0,
            },
            4,
            self.round_timer,
        );

        // Buttons
        if self.gui.button(
            Rect {
                x: screen_width() / 2.0 - 18.0,
                y: 12.0,
                w: 144.0,
                h: 32.0,
            },
            "MENU",
        ) {
            self.state = GameState::InMenu;
        }

        if self.gui.button(
            Rect {
                x: screen_width() / 2.0 - 18.0,
                y: 52.0,
                w: 144.0,
                h: 32.0,
            },
            "NEW GAME",
        ) {
            self.gen_field(self.field_width, self.field_height, self.field_mines);

            self.round_start = 0;
            self.round_timer = 0;
            self.do_timer = false;
            self.status_face = StatusFace::Happy;
            self.game_over = false;
            self.game_won = false;
        }
    }

    fn draw_minefield(&self) {
        for x in 0..self.field_width {
            for y in 0..self.field_height {
                let i = y * self.field_width + x;
                let field_cell = self.field[i as usize].clone();
                let cover_cell = self.cover[i as usize].clone();

                draw_texture_rec(
                    &self.assets.textures.field_cells,
                    Vec2 {
                        x: (x as f32) * 32.0,
                        y: 120.0 + (y as f32) * 32.0,
                    },
                    Rect {
                        x: (field_cell as i32) as f32 * 32.0,
                        y: 0.0,
                        w: 32.0,
                        h: 32.0,
                    },
                    WHITE,
                );

                draw_texture_rec(
                    &self.assets.textures.top_cells,
                    Vec2 {
                        x: (x as f32) * 32.0,
                        y: 120.0 + (y as f32) * 32.0,
                    },
                    Rect {
                        x: (cover_cell as i32) as f32 * 32.0,
                        y: 0.0,
                        w: 32.0,
                        h: 32.0,
                    },
                    WHITE,
                );
            }
        }
    }

    pub fn do_minefield(&mut self) {
        if self.do_timer {
            self.round_timer = (get_time() as i32) - self.round_start;
        }

        self.draw_bg();
        self.draw_status_bar();
        self.draw_minefield();

        // Count statistics
        let mut revealed_cells = 0;

        if !self.game_over {
            self.placed_flags = 0;

            for i in 0..(self.field_width * self.field_height) as usize {
                if matches!(self.cover[i], CoverCell::None) {
                    revealed_cells += 1;
                };
                if matches!(self.cover[i], CoverCell::Flag) {
                    self.placed_flags += 1;
                };
            }
        }

        // Check if player won
        if (revealed_cells == (self.field_width * self.field_height) - self.field_mines)
            && (!self.game_won)
        {
            self.game_won = true;
            self.do_timer = false;
            self.status_face = StatusFace::Cool;
            play_sound_once(&self.assets.sounds.win)
        }

        // Main logic
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;
        let selection_x = (mouse_x / 32.0).floor();
        let selection_y = ((mouse_y - 120.0) / 32.0).floor();
        let selection_pos = ((selection_y * self.field_width as f32) + selection_x) as usize;

        if (mouse_y > 120.0)
            && (mouse_y < (self.field_height as f32) * 32.0 + 120.0)
            && (mouse_x < (self.field_width as f32) * 32.0)
            && (!(self.game_won || self.game_over))
        {
            // Drawing selection
            draw_rectangle_lines(
                selection_x * 32.0,
                selection_y * 32.0 + 120.0,
                32.0,
                32.0,
                2.0,
                color_u8!(0x00, 0x71, 0x00, 0xff),
            );

            draw_rectangle_lines(
                selection_x * 32.0 + 1.0,
                selection_y * 32.0 + 121.0,
                30.0,
                30.0,
                2.0,
                color_u8!(0x00, 0xaa, 0x00, 0xff),
            );

            // Mouse actions
            if is_mouse_button_down(MouseButton::Left) {
                self.status_face = StatusFace::Surprised;
            } else {
                self.status_face = StatusFace::Happy;
            }

            // When a player clicks to reveal a mine...
            if is_mouse_button_pressed(MouseButton::Left) {
                // If there is a flag, remove it rather than revealing the cell below.
                if matches!(self.cover[selection_pos], CoverCell::Flag) {
                    self.cover[selection_pos] = CoverCell::Blank;
                }
                // Otherwise, if the cell has never been clicked before...
                else if matches!(self.cover[selection_pos], CoverCell::Blank) {
                    // Generate new fields until the place they clicked contains no mines.
                    while matches!(self.field[selection_pos], FieldCell::Mine)
                        && (self.clicked_cells == 0)
                    {
                        self.gen_field(self.field_width, self.field_height, self.field_mines);
                    }

                    // Start timer if it hasn't started.
                    if !self.do_timer {
                        self.round_start = get_time() as i32;
                        self.do_timer = true;
                    }

                    self.clicked_cells += 1;

                    // Handle each condition for possibly clicked cells
                    match self.field[selection_pos] {
                        FieldCell::Mine => {
                            self.game_over = true;
                            self.do_timer = false;
                            self.status_face = StatusFace::Dead;

                            // Iterate through each field of the mine.
                            for i in 0..(self.field_width * self.field_height) as usize {
                                // Reveal each mine
                                if matches!(self.field[i], FieldCell::Mine) {
                                    self.cover[i] = CoverCell::None;
                                }

                                // Handle flag placements
                                if matches!(self.cover[i], CoverCell::Flag) {
                                    self.cover[i] = match self.field[i] {
                                        FieldCell::Mine => CoverCell::None,
                                        _ => CoverCell::WrongFlag,
                                    };
                                }
                            }

                            // Highlight cell & create immersive gameplay
                            self.cover[selection_pos] = CoverCell::CurrentMine;
                            play_sound_once(&self.assets.sounds.explode); // <-- Immersive gameplay created here.
                        }
                        FieldCell::Empty => {
                            self.reveal_empty_cells(selection_x as i32, selection_y as i32);
                        }
                        _ => {
                            self.cover[selection_pos] = CoverCell::None;
                        }
                    }
                }
            } else if is_mouse_button_pressed(MouseButton::Right) {
                // Start timer if it hasn't started.
                if !self.do_timer {
                    self.round_start = get_time() as i32;
                    self.do_timer = true;
                }

                // Place or remove flag.
                self.cover[selection_pos] = match self.cover[selection_pos] {
                    CoverCell::Blank => CoverCell::Flag,
                    CoverCell::Flag => CoverCell::Blank,
                    _ => self.cover[selection_pos].clone(),
                };
            }
        }
    }
}
