use ext_gfx::drawing::*;
use ext_gfx::gui::*;
use macroquad::miniquad::conf::Icon;
use macroquad::{audio::*, miniquad::window::*, prelude::*, rand::*};
use std::cmp;

pub mod ext_gfx;
pub mod screens;

struct GameTextures {
    menu_background: Texture2D,
    field_cells: Texture2D,
    top_cells: Texture2D,
    status_bar: Texture2D,
}

struct GameSounds {
    explode: Sound,
    win: Sound,
}

struct GameAssets {
    theme_name: String,
    textures: GameTextures,
    sounds: GameSounds,
}

enum GameState {
    InMenu,
    InRound,
}

#[derive(Clone)]
enum FieldCell {
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Mine,
}

impl FieldCell {
    fn try_from(v: i32) -> Result<Self, &'static str> {
        match v {
            x if x == FieldCell::Empty as i32 => Ok(FieldCell::Empty),
            x if x == FieldCell::One as i32 => Ok(FieldCell::One),
            x if x == FieldCell::Two as i32 => Ok(FieldCell::Two),
            x if x == FieldCell::Three as i32 => Ok(FieldCell::Three),
            x if x == FieldCell::Four as i32 => Ok(FieldCell::Four),
            x if x == FieldCell::Five as i32 => Ok(FieldCell::Five),
            x if x == FieldCell::Six as i32 => Ok(FieldCell::Six),
            x if x == FieldCell::Seven as i32 => Ok(FieldCell::Seven),
            x if x == FieldCell::Eight as i32 => Ok(FieldCell::Eight),
            x if x == FieldCell::Mine as i32 => Ok(FieldCell::Mine),
            _ => Err("Number outside range of enum."),
        }
    }
}

#[derive(Clone)]
enum CoverCell {
    None,
    Blank,
    Flag,
    CurrentMine,
    WrongFlag,
}

enum StatusFace {
    Happy,
    Surprised,
    Cool,
    Dead,
}

impl StatusFace {
    fn get_texture_rect(&self) -> Rect {
        match *self {
            StatusFace::Happy => Rect {
                x: 0.0,
                y: 0.0,
                w: 96.0,
                h: 96.0,
            },
            StatusFace::Surprised => Rect {
                x: 96.0,
                y: 0.0,
                w: 96.0,
                h: 96.0,
            },
            StatusFace::Cool => Rect {
                x: 0.0,
                y: 96.0,
                w: 96.0,
                h: 96.0,
            },
            StatusFace::Dead => Rect {
                x: 96.0,
                y: 96.0,
                w: 96.0,
                h: 96.0,
            },
        }
    }
}

enum StatusSegment {
    Blank,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Colon,
    Mine,
}

impl StatusSegment {
    fn try_from(v: i32) -> Result<Self, String> {
        match v + 1 {
            x if x == StatusSegment::Zero as i32 => Ok(StatusSegment::Zero),
            x if x == StatusSegment::One as i32 => Ok(StatusSegment::One),
            x if x == StatusSegment::Two as i32 => Ok(StatusSegment::Two),
            x if x == StatusSegment::Three as i32 => Ok(StatusSegment::Three),
            x if x == StatusSegment::Four as i32 => Ok(StatusSegment::Four),
            x if x == StatusSegment::Five as i32 => Ok(StatusSegment::Five),
            x if x == StatusSegment::Six as i32 => Ok(StatusSegment::Six),
            x if x == StatusSegment::Seven as i32 => Ok(StatusSegment::Seven),
            x if x == StatusSegment::Eight as i32 => Ok(StatusSegment::Eight),
            x if x == StatusSegment::Nine as i32 => Ok(StatusSegment::Nine),
            _ => Err(format!("Number outside range of enum ({v}).")),
        }
    }

    fn get_texture_rect(&self) -> Rect {
        match *self {
            StatusSegment::Blank => Rect {
                x: 192.0,
                y: 0.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Zero => Rect {
                x: 224.0,
                y: 0.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::One => Rect {
                x: 256.0,
                y: 0.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Two => Rect {
                x: 288.0,
                y: 0.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Three => Rect {
                x: 192.0,
                y: 56.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Four => Rect {
                x: 224.0,
                y: 56.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Five => Rect {
                x: 256.0,
                y: 56.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Six => Rect {
                x: 288.0,
                y: 56.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Seven => Rect {
                x: 192.0,
                y: 112.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Eight => Rect {
                x: 224.0,
                y: 112.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Nine => Rect {
                x: 256.0,
                y: 112.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Colon => Rect {
                x: 288.0,
                y: 112.0,
                w: 32.0,
                h: 56.0,
            },
            StatusSegment::Mine => Rect {
                x: 320.0,
                y: 0.0,
                w: 32.0,
                h: 56.0,
            },
        }
    }
}

struct Game {
    assets: GameAssets,
    gui: GUI,
    state: GameState,
    field_width: i32,
    field_height: i32,
    field_mines: i32,
    clicked_cells: i32,
    placed_flags: i32,
    round_start: i32,
    round_timer: i32,
    status_face: StatusFace,
    game_over: bool,
    game_won: bool,
    do_timer: bool,
    should_quit: bool,
    field: Vec<FieldCell>,
    cover: Vec<CoverCell>,
}

impl Game {
    async fn start(&mut self) {
        loop {
            if self.should_quit {
                quit()
            }

            match self.state {
                GameState::InMenu => self.do_titlemenu(),
                GameState::InRound => self.do_minefield(),
            }

            if is_key_pressed(KeyCode::Q) {
                self.load_theme("classic").await;
            } else if is_key_pressed(KeyCode::W) {
                self.load_theme("marble").await;
            }

            next_frame().await
        }
    }

    fn count_cell_neighbours(&self, x: i32, y: i32) -> i32 {
        if (x < 0) || (y < 0) || (x >= self.field_width) || (y >= self.field_height) {
            return -1;
        };

        // Count mines
        let mut mines = 0;
        let initpos: usize = (y * self.field_width + x) as usize;

        if matches!(self.field[initpos], FieldCell::Mine) {
            return 9; // Return 9 if on mine.
        };

        // Check each cell surrounding the starting cell
        for xoff in -1..=1 {
            for yoff in -1..=1 {
                let pos: usize = (((y + yoff) * self.field_width) + (x + xoff)) as usize; // Calculate the index into the field array

                // Skip out of bounds cells
                if ((x + xoff) < 0)
                    || ((y + yoff) < 0)
                    || ((x + xoff) >= self.field_width)
                    || ((y + yoff) >= self.field_height)
                {
                    continue;
                } else if matches!(self.field[pos], FieldCell::Mine) {
                    mines += 1;
                }
            }
        }

        mines
    }

    fn reveal_empty_cells(&mut self, x: i32, y: i32) {
        let idx: usize = ((y * self.field_width) + x) as usize;

        if (x < 0) || (y < 0) || (x >= self.field_width) || (y >= self.field_height) {
            return;
        }

        let cell_neighbours = self.count_cell_neighbours(x, y);

        if (cell_neighbours == 0) && !matches!(self.cover[idx], CoverCell::None) {
            self.cover[idx] = CoverCell::None;

            self.reveal_empty_cells(x, y - 1);
            self.reveal_empty_cells(x, y + 1);
            self.reveal_empty_cells(x - 1, y);
            self.reveal_empty_cells(x + 1, y);
        } else if cell_neighbours != -1 {
            self.cover[idx] = CoverCell::None;
        }
    }

    fn gen_field(&mut self, width: i32, height: i32, mines: i32) {
        set_window_size(
            cmp::max((width * 32) as u32, 640),
            cmp::max((height * 32 + 120) as u32, 440),
        );

        // Set some game states
        self.field_width = width;
        self.field_height = height;
        self.field_mines = mines;

        self.game_over = false;
        self.clicked_cells = 0;

        // Replace arrays
        let area = width * height;
        self.field = vec![FieldCell::Empty; area as usize];
        self.cover = vec![CoverCell::Blank; area as usize];

        // Generate mines
        let mut placed_mines = 0;

        while placed_mines != mines {
            let rx = gen_range(0, self.field_width);
            let ry = gen_range(0, self.field_height);
            let i = ((ry * self.field_width) + rx) as usize;

            if !matches!(self.field[i], FieldCell::Mine) {
                self.field[i] = FieldCell::Mine;
                placed_mines += 1;
            }
        }

        // Place numbers
        for x in 0..self.field_width {
            for y in 0..self.field_height {
                let idx = (y * self.field_width + x) as usize;
                self.field[idx] = FieldCell::try_from(self.count_cell_neighbours(x, y))
                    .unwrap_or(FieldCell::Empty);
            }
        }
    }

    async fn load_theme(&mut self, name: &str) {
        let resource_path = format!("./resources/themes/{name}");

        let textures = GameTextures {
            menu_background: load_texture(format!("{resource_path}/textures/splash.png").as_str())
                .await
                .unwrap(),
            field_cells: load_texture(format!("{resource_path}/textures/fieldcells.png").as_str())
                .await
                .unwrap(),
            top_cells: load_texture(format!("{resource_path}/textures/topcells.png").as_str())
                .await
                .unwrap(),
            status_bar: load_texture(format!("{resource_path}/textures/statusbar.png").as_str())
                .await
                .unwrap(),
        };

        let sounds = GameSounds {
            explode: load_sound(format!("{resource_path}/sounds/explode.ogg").as_str())
                .await
                .unwrap(),
            win: load_sound(format!("{resource_path}/sounds/win.ogg").as_str())
                .await
                .unwrap(),
        };

        self.assets.textures = textures;
        self.assets.sounds = sounds;

        self.gui = GUI::new(name).await;
    }

    async fn new(width: u32, height: u32) -> Game {
        set_window_size(width, height);

        let assets = GameAssets {
            theme_name: String::from("classic"),
            textures: GameTextures {
                menu_background: load_texture("./resources/themes/classic/textures/splash.png")
                    .await
                    .unwrap(),
                field_cells: load_texture("./resources/themes/classic/textures/fieldcells.png")
                    .await
                    .unwrap(),
                top_cells: load_texture("./resources/themes/classic/textures/topcells.png")
                    .await
                    .unwrap(),
                status_bar: load_texture("./resources/themes/classic/textures/statusbar.png")
                    .await
                    .unwrap(),
            },
            sounds: GameSounds {
                explode: load_sound("./resources/themes/classic/sounds/explode.ogg")
                    .await
                    .unwrap(),
                win: load_sound("./resources/themes/classic/sounds/win.ogg")
                    .await
                    .unwrap(),
            },
        };

        let mut game = Game {
            assets,
            gui: GUI::new("classic").await,
            state: GameState::InMenu,
            field_width: 1,
            field_height: 1,
            field_mines: 0,
            clicked_cells: 0,
            placed_flags: 0,
            round_start: 0,
            round_timer: 0,
            status_face: StatusFace::Happy,
            game_over: false,
            game_won: false,
            do_timer: false,
            should_quit: false,
            field: vec![FieldCell::Empty; 1],
            cover: vec![CoverCell::Blank; 1],
        };

        game.load_theme("classic").await;

        game
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    Game::new(800, 600).await.start().await
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Mines!"),
        icon: Some(Icon::miniquad_logo()),
        ..Default::default()
    }
}
