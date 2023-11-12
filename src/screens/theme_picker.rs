use crate::*;

impl Game {
    pub fn do_theme_picker(&mut self) {
        self.draw_bg();

        if self.gui.button(
            Rect {
                x: 256.0,
                y: 16.0,
                w: 288.0,
                h: 32.0,
            },
            "<< Back",
        ) {
            self.state = GameState::InMenu;
        }

        if self.gui.button(
            Rect {
                x: 256.0,
                y: 64.0,
                w: 288.0,
                h: 32.0,
            },
            "Reset",
        ) {
            self.state = GameState::InMenu;
        }
    }
}
