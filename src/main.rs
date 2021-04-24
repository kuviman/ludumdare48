use geng::prelude::*;

#[derive(geng::Assets)]
pub struct Assets {
    art: ugli::Texture,
}

struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
}

impl GameState {
    pub fn new(geng: &Rc<Geng>, assets: Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets,
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::rgb(0.8, 0.8, 1.0)), None);
    }
    fn update(&mut self, delta_time: f64) {}
    fn handle_event(&mut self, event: geng::Event) {}
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }
    let geng = Rc::new(Geng::new(geng::ContextOptions {
        title: "LudumDare 48 - TODO by kuviman".to_owned(),
        ..default()
    }));
    let assets = <Assets as geng::LoadAsset>::load(&geng, ".");
    geng::run(
        geng.clone(),
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let assets = assets.unwrap();
                GameState::new(&geng, Rc::new(assets))
            }
        }),
    );
}
