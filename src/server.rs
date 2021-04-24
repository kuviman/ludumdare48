use super::*;

struct ServerState {
    model: Model,
    events: std::collections::VecDeque<Event>,
    next_event_index: usize,
    first_event_index: usize,
    clients_next_event: HashMap<Id, usize>,
}

impl ServerState {
    fn new(model: Model) -> Self {
        Self {
            model,
            events: default(),
            next_event_index: 0,
            first_event_index: 0,
            clients_next_event: default(),
        }
    }
    fn add_events(&mut self, events: impl IntoIterator<Item = Event>) {
        for event in events.into_iter() {
            // eprintln!("Add {}: {:?}", self.next_event_index, event);
            self.events.push_back(event);
            self.next_event_index += 1;
        }
    }
    fn shrink(&mut self) {
        let next_needed_event_index = self
            .clients_next_event
            .values()
            .map(|&index| index)
            .min()
            .unwrap_or(self.next_event_index);
        while self.first_event_index < next_needed_event_index {
            self.events.pop_front();
            self.first_event_index += 1;
        }
    }
    fn get_new_events(&mut self, player_id: Id) -> Vec<Event> {
        let next_event_index = self.clients_next_event[&player_id];
        let mut result = Vec::new();
        for index in next_event_index..self.next_event_index {
            result.push(self.events[index - self.first_event_index].clone());
        }
        self.clients_next_event
            .insert(player_id, self.next_event_index);
        self.shrink();
        // eprintln!("{:?}: {:?}", player_id, result);
        result
    }
}

struct Client {
    player_id: Id,
    server_state: Arc<Mutex<ServerState>>,
    sender: Box<dyn geng::net::Sender<ServerMessage>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        let mut server_state = self.server_state.lock().unwrap();
        let events = server_state.model.drop_player(self.player_id);
        server_state.add_events(events);
    }
}

impl geng::net::Receiver<ClientMessage> for Client {
    fn handle(&mut self, message: ClientMessage) {
        let send_update = matches!(message, ClientMessage::Update { .. });
        let mut server_state = self.server_state.lock().unwrap();
        let events = server_state.model.handle_message(
            self.player_id,
            message,
            // &mut *self.sender
        );
        server_state.add_events(events);
        if send_update {
            self.sender.send(ServerMessage::Update(
                server_state.get_new_events(self.player_id),
            ));
        }
    }
}
struct ServerApp {
    server_state: Arc<Mutex<ServerState>>,
}
impl geng::net::server::App for ServerApp {
    type Client = Client;
    type ServerMessage = ServerMessage;
    type ClientMessage = ClientMessage;
    fn connect(&mut self, mut sender: Box<dyn geng::net::Sender<ServerMessage>>) -> Client {
        let mut server_state = self.server_state.lock().unwrap();
        let (welcome, events) = server_state.model.welcome();
        server_state.add_events(events);
        let player_id = welcome.player_id;
        sender.send(ServerMessage::Welcome(welcome));
        let next_event_index = server_state.next_event_index;
        server_state
            .clients_next_event
            .insert(player_id, next_event_index);
        sender.send(ServerMessage::Update(vec![]));
        Client {
            server_state: self.server_state.clone(),
            player_id,
            sender,
        }
    }
}

pub struct Server {
    server_state: Arc<Mutex<ServerState>>,
    server: geng::net::Server<ServerApp>,
}

impl Server {
    pub fn new<T: std::net::ToSocketAddrs + Debug + Copy>(addr: T, model: Model) -> Self {
        let server_state = Arc::new(Mutex::new(ServerState::new(model)));
        Self {
            server_state: server_state.clone(),
            server: geng::net::Server::new(
                ServerApp {
                    server_state: server_state.clone(),
                },
                addr,
            ),
        }
    }
    pub fn handle(&self) -> geng::net::ServerHandle {
        self.server.handle()
    }
    pub fn run(self) {
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let server_thread = std::thread::spawn({
            let server_state = self.server_state;
            let running = running.clone();
            let mut sleep_time = 0;
            move || {
                while running.load(std::sync::atomic::Ordering::Relaxed) {
                    // TODO: smoother TPS
                    std::thread::sleep(std::time::Duration::from_millis(sleep_time));
                    let mut server_state = server_state.lock().unwrap();
                    let events = server_state.model.tick();
                    server_state.add_events(events);
                    sleep_time = (1000.0 / server_state.model.ticks_per_second) as u64;
                }
            }
        });
        self.server.run();
        running.store(false, std::sync::atomic::Ordering::Relaxed);
        server_thread.join().expect("Failed to join server thread");
    }
}

// pub struct Server {
//     handle: geng::net::ServerHandle,
//     thread: Option<std::thread::JoinHandle<()>>,
// }

// impl Server {
//     pub fn new() -> Self {
//         let server = ServerState::new(SERVER_ADDR, Model::new());
//         let handle = server.handle();
//         let thread = std::thread::spawn(move || server.run());
//         Self {
//             handle,
//             thread: Some(thread),
//         }
//     }
// }

// impl Drop for Server {
//     fn drop(&mut self) {
//         self.handle.shutdown();
//         self.thread
//             .take()
//             .unwrap()
//             .join()
//             .expect("Failed to join server thread");
//     }
// }
