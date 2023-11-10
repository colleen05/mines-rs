use macroquad::{audio::*, miniquad::window::set_window_size, prelude::*};

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
    icon: Image,
    textures: GameTextures,
    sounds: GameSounds,
}

struct Game {
    assets: GameAssets,
}

impl Game {
    fn draw(&mut self) {}

    fn update(&mut self) {
        draw_texture(&self.assets.textures.menu_background, 0.0, 0.0, WHITE);
    }

    async fn start(&mut self) {
        loop {
            self.update();
            self.draw();

            next_frame().await
        }
    }

    async fn new(width: u32, height: u32) -> Game {
        set_window_size(width, height);

        let assets = GameAssets {
            icon: load_image("./resources/icon.png").await.unwrap(),
            textures: GameTextures {
                menu_background: load_texture("./resources/textures/splash.png")
                    .await
                    .unwrap(),
                field_cells: load_texture("./resources/textures/fieldcells.png")
                    .await
                    .unwrap(),
                top_cells: load_texture("./resources/textures/topcells.png")
                    .await
                    .unwrap(),
                status_bar: load_texture("./resources/textures/statusbar.png")
                    .await
                    .unwrap(),
            },
            sounds: GameSounds {
                explode: load_sound("./resources/sounds/explode.ogg").await.unwrap(),
                win: load_sound("./resources/sounds/win.ogg").await.unwrap(),
            },
        };

        Game { assets }
    }
}

#[macroquad::main("Mines")]
async fn main() {
    Game::new(800, 600).await.start().await
}
