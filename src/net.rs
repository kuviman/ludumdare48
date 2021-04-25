use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Event(Event),
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
