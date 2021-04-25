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
pub use lobby::Lobby;
pub use model::*;
pub use net::*;
pub use renderer::*;
#[cfg(not(target_arch = "wasm32"))]
pub use server::Server;

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
    pub eyes: ugli::Texture,
    pub border: ugli::Texture,
    pub grass: ugli::Texture,
    #[asset(path = "block.png", range = "0..1")]
    pub block: Vec<ugli::Texture>,
    pub block_item: ugli::Texture,
    pub ladder_item: ugli::Texture,
    pub chest: ugli::Texture,
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
                    Lobby::new(&geng, Rc::new(assets), &opt)
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
