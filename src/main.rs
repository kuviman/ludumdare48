use geng::prelude::*;

pub mod camera;
pub mod game_state;
pub mod lobby;
pub mod model;
pub mod net;
pub mod renderer;
#[cfg(not(target_arch = "wasm32"))]
pub mod server;

pub use camera::*;
pub use game_state::GameState;
pub use lobby::*;
pub use model::*;
pub use net::*;
pub use renderer::*;
#[cfg(not(target_arch = "wasm32"))]
pub use server::Server;

pub fn hsv(h: f32, s: f32, v: f32) -> Color<f32> {
    hsva(h, s, v, 1.0)
}
pub fn hsva(mut h: f32, s: f32, v: f32, a: f32) -> Color<f32> {
    h -= h.floor();
    let r;
    let g;
    let b;
    let f = h * 6.0 - (h * 6.0).floor();
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    if h * 6.0 < 1.0 {
        r = v;
        g = t;
        b = p;
    } else if h * 6.0 < 2.0 {
        r = q;
        g = v;
        b = p;
    } else if h * 6.0 < 3.0 {
        r = p;
        g = v;
        b = t;
    } else if h * 6.0 < 4.0 {
        r = p;
        g = q;
        b = v;
    } else if h * 6.0 < 5.0 {
        r = t;
        g = p;
        b = v;
    } else {
        r = v;
        g = p;
        b = q;
    }
    Color::rgba(r, g, b, a)
}

#[derive(geng::Assets)]
pub struct Assets {
    pub art: ugli::Texture,
    pub player: ugli::Texture,
    #[asset(path = "stone/*.png", range = "1..=6")]
    pub stone: Vec<ugli::Texture>,
    #[asset(path = "ladder/*.png", range = "1..=2")]
    pub ladder: Vec<ugli::Texture>,
    pub stick: ugli::Texture,
    pub pick_head: ugli::Texture,
    pub body: ugli::Texture,
    pub leg: ugli::Texture,
    pub border: ugli::Texture,
    pub grass: ugli::Texture,
    #[asset(path = "block.png", range = "0..1")]
    pub block: Vec<ugli::Texture>,
    pub block_item: ugli::Texture,
    pub ladder_item: ugli::Texture,
    pub chest: ugli::Texture,
    pub sell_shop: ugli::Texture,
    pub combine_shop: ugli::Texture,
    pub coin: ugli::Texture,
    pub house: ugli::Texture,
    pub train: ugli::Texture,
    #[asset(path = "eye/*.png", range = "1..=4")]
    pub eye: Vec<ugli::Texture>,
    #[asset(path = "beard/*.png", range = "1..=4")]
    pub beard: Vec<ugli::Texture>,
    #[asset(path = "ear/*.png", range = "1..=4")]
    pub ear: Vec<ugli::Texture>,
    #[asset(path = "hat/*.png", range = "1..=4")]
    pub hat: Vec<ugli::Texture>,
    #[asset(path = "mouth/*.png", range = "1..=4")]
    pub mouth: Vec<ugli::Texture>,
    #[asset(path = "mustache/*.png", range = "1..=4")]
    pub mustache: Vec<ugli::Texture>,
    #[asset(path = "nose/*.png", range = "1..=4")]
    pub nose: Vec<ugli::Texture>,
}

impl Assets {
    pub fn item_texture(&self, item_type: ItemType) -> &ugli::Texture {
        match item_type {
            ItemType::Block => &self.block_item,
            ItemType::Chest => &self.chest,
            ItemType::Ladder => &self.ladder_item,
        }
    }
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long)]
    addr: Option<String>,
    #[structopt(long)]
    server: bool,
    #[structopt(long)]
    with_server: bool,
}

impl Opt {
    pub fn addr(&self) -> &str {
        match &self.addr {
            Some(addr) => addr,
            None => option_env!("SERVER_ADDR").unwrap_or("127.0.0.1:1155"),
        }
    }
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
    let opt: Opt = StructOpt::from_args();
    let opt = Rc::new(opt);
    if opt.server {
        #[cfg(not(target_arch = "wasm32"))]
        Server::new(opt.addr(), Model::new()).run();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let server = if opt.with_server {
            let server = Server::new(opt.addr(), Model::new());
            let server_handle = server.handle();
            let server_thread = std::thread::spawn(move || {
                server.run();
            });
            Some((server_handle, server_thread))
        } else {
            None
        };
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
                    let mut model = Model::new();
                    let (welcome, _) = model.welcome();
                    GameState::new(
                        &geng,
                        &Rc::new(assets),
                        &opt,
                        None,
                        welcome,
                        Connection::Local {
                            next_tick: 0.0,
                            model,
                        },
                    )
                    // Lobby::new(&geng, Rc::new(assets), &opt)
                }
            }),
        );
        #[cfg(not(target_arch = "wasm32"))]
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
