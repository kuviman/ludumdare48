use super::*;

pub const SERVER_ADDR: &str = "127.0.0.1:7897";

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Update { position: Vec2<f32> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeMessage {
    pub player_id: Id,
    pub model: Model,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Welcome(WelcomeMessage),
    Update(Vec<Event>),
}

pub enum Connection {
    Local { next_tick: f64, model: Model },
    Remote(geng::net::client::Connection<ServerMessage, ClientMessage>),
}
