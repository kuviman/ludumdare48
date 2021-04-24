use geng::prelude::*;

pub mod camera;
pub mod game_state;
pub mod lobby;
pub mod model;
pub mod net;
pub mod renderer;
pub mod server;

pub use camera::*;
pub use game_state::GameState;
pub use lobby::Lobby;
pub use model::*;
pub use net::*;
pub use renderer::*;
pub use server::Server;

#[derive(geng::Assets)]
pub struct Assets {
    pub art: ugli::Texture,
    pub player: ugli::Texture,
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(long)]
    server: bool,
    #[structopt(long)]
    with_server: bool,
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
    if opt.server {
        Server::new(SERVER_ADDR, Model::new()).run();
    } else {
        let server = if opt.with_server {
            let server = Server::new(SERVER_ADDR, Model::new());
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
                    Lobby::new(&geng, Rc::new(assets))
                }
            }),
        );
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
