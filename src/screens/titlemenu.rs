use crate::*;

impl Game {
    pub fn do_titlemenu(&mut self) {
        // Reset state
        self.do_timer = false;
        self.round_timer = 0;
        self.round_start = 0;
        self.game_won = false;
        self.game_over = false;
        self.status_face = StatusFace::Happy;

        let mut selected = false;
        let mut sel_size = (1, 1);
        let mut sel_mines = 1;

        // Draw background
        draw_texture(&self.assets.textures.menu_background, 0.0, 0.0, WHITE);

        // Preset buttons
        const PRESETS: [(&str, i32, i32, i32); 3] = [
            ("BEGINNER", 10, 10, 10),  // Beginner   (10x10 | 10 mines)
            ("DIFFICULT", 25, 15, 50), // Difficult  (25x15 | 50 mines)
            ("EXPERT", 30, 15, 99),    // Expert     (30x15 | 99 mines)
        ];

        for (i, preset) in PRESETS.iter().enumerate() {
            let x: f32 = 96.0;
            let y: f32 = 284.0 + 48.0 * (i as f32);
            let w: f32 = 144.0;
            let h: f32 = 32.0;

            draw_text(
                &format!("{}x{}|{} Mines", preset.1, preset.2, preset.3),
                x + 156.0,
                y + 24.0,
                32.0,
                color_u8!(0x00, 0x00, 0x00, 0x7f),
            );

            if self.gui.button(Rect { x, y, w, h }, preset.0) {
                selected = true;
                sel_size = (preset.1, preset.2);
                sel_mines = preset.3;
            }
        }

        // Custom selection
        self.field_width = self.gui.spinner(536.0, 284.0, self.field_width, 5, 50);
        self.field_height = self.gui.spinner(636.0, 284.0, self.field_height, 5, 25);

        let maxmines = clamp(self.field_width * self.field_height - 1, 1, 999);

        self.field_mines = self
            .gui
            .spinner(636.0, 332.0, self.field_mines, 1, maxmines);

        if self.gui.button(
            Rect {
                x: 496.0,
                y: 380.0,
                w: 208.0,
                h: 32.0,
            },
            "CUSTOM",
        ) {
            selected = true;
            sel_size = (self.field_width, self.field_height);
            sel_mines = self.field_mines;
        }

        // Handle selection
        if selected {
            self.gen_field(sel_size.0, sel_size.1, sel_mines);

            self.round_start = get_time() as i32;
            self.clicked_cells = 0;
            self.state = GameState::InRound;
        }
    }
}
