use raylib::prelude::*;

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
    rl: RaylibHandle,
    thread: RaylibThread,
    assets: GameAssets,
}

impl Game {
    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&self.assets.textures.menu_background, 0, 0, Color::WHITE)
    }

    fn update(&mut self) {}

    fn start(&mut self) {
        self.rl.set_window_icon(&self.assets.icon);

        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }

    fn new(title: &str, width: i32, height: i32) -> Game {
        let (mut rl, thread) = raylib::init().size(width, height).title(title).build();
        let assets = GameAssets {
            icon: Image::load_image("./resources/icon.png").expect("Could not load image."),
            textures: GameTextures {
                menu_background: rl
                    .load_texture(&thread, "./resources/textures/splash.png")
                    .expect("Could not load texture."),
                field_cells: rl
                    .load_texture(&thread, "./resources/textures/fieldcells.png")
                    .expect("Could not load texture."),
                top_cells: rl
                    .load_texture(&thread, "./resources/textures/topcells.png")
                    .expect("Could not load texture."),
                status_bar: rl
                    .load_texture(&thread, "./resources/textures/statusbar.png")
                    .expect("Could not load texture."),
            },
            sounds: GameSounds {
                explode: Sound::load_sound("./resources/sounds/explode.ogg")
                    .expect("Could not load sound."),
                win: Sound::load_sound("./resources/sounds/win.ogg")
                    .expect("Could not load sound."),
            },
        };
        Game { rl, thread, assets }
    }
}

fn main() {
    Game::new("Mines", 800, 600).start();
}
