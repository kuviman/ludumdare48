use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Id(usize);

impl Id {
    pub fn raw(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdGen {
    next_id: usize,
}

impl IdGen {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    pub fn gen(&mut self) -> Id {
        let id = Id(self.next_id);
        self.next_id += 1;
        id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Id,
    pub position: Vec2<f32>,
    pub target_velocity: Vec2<f32>,
    pub size: Vec2<f32>,
    pub jump_timer: f32,
}

impl Player {
    pub const SPEED: f32 = 3.0;
    pub const JUMP_SPEED: f32 = 4.0;
    pub const JUMP_TIME: f32 = 0.3;
    pub fn new(id_gen: &mut IdGen) -> Self {
        Self {
            id: id_gen.gen(),
            position: vec2(0.0, 0.0),
            target_velocity: vec2(0.0, 0.0),
            size: vec2(0.5, 0.5),
            jump_timer: 0.0,
        }
    }
    pub fn matrix(&self) -> Mat4<f32> {
        Mat4::translate(self.position.extend(0.0))
            * Mat4::scale(vec3(self.size.x, self.size.y, 1.0))
    }
    pub fn tiles(&self) -> impl Iterator<Item = Vec2<i32>> + '_ {
        (self.position.x.floor() as i32..(self.position.x + self.size.x).ceil() as i32).flat_map(
            move |x| {
                (self.position.y.floor() as i32..(self.position.y + self.size.y).ceil() as i32)
                    .map(move |y| vec2(x, y))
            },
        )
    }
    pub fn update(&mut self, tiles: &TileMap, delta_time: f32) {
        let initial_position = self.position;
        let mut velocity = self.target_velocity;
        velocity.x *= Self::SPEED;
        if velocity.y != 1.0 {
            self.jump_timer = 0.0;
        }
        if self.jump_timer <= 0.0 {
            velocity.y = -1.0;
        }
        velocity.y *= Self::JUMP_SPEED;
        self.jump_timer -= delta_time;
        let delta_position = velocity * delta_time;
        self.position.x += delta_position.x;
        if self.collide(tiles) {
            self.position.x = initial_position.x;
        }
        self.position.y += delta_position.y;
        if self.collide(tiles) {
            if self.position.y < initial_position.y {
                self.jump_timer = Self::JUMP_TIME;
            } else {
                self.jump_timer = 0.0;
            }
            self.position.y = initial_position.y;
        }
    }
    fn collide(&self, tiles: &TileMap) -> bool {
        for position in self.tiles() {
            if tiles.contains_key(&position) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tile {
    Stone,
}

pub type TileMap = HashMap<Vec2<i32>, Tile>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    id_gen: IdGen,
    pub ticks_per_second: f64,
    pub players: HashMap<Id, Player>,
    pub tiles: TileMap,
}

impl Model {
    pub fn new() -> Self {
        Self {
            id_gen: IdGen::new(),
            ticks_per_second: 20.0,
            players: default(),
            tiles: {
                let mut tiles = TileMap::new();
                for x in -100..=100 {
                    for y in -100..0 {
                        tiles.insert(vec2(x, y), Tile::Stone);
                    }
                }
                tiles
            },
        }
    }
    #[must_use]
    fn spawn_player(&mut self) -> (Id, Vec<Event>) {
        let player = Player::new(&mut self.id_gen);
        let events = vec![Event::PlayerJoined(player.clone())];
        let player_id = player.id;
        self.players.insert(player_id, player);
        (player_id, events)
    }
    #[must_use]
    pub fn welcome(&mut self) -> (WelcomeMessage, Vec<Event>) {
        let (player_id, events) = self.spawn_player();
        (
            WelcomeMessage {
                player_id,
                model: self.clone(),
            },
            events,
        )
    }
    #[must_use]
    pub fn drop_player(&mut self, player_id: Id) -> Vec<Event> {
        self.players.remove(&player_id).unwrap();
        vec![Event::PlayerLeft(player_id)]
    }
    #[must_use]
    pub fn handle_message(
        &mut self,
        player_id: Id,
        message: ClientMessage,
        // sender: &mut dyn geng::net::Sender<ServerMessage>,
    ) -> Vec<Event> {
        let mut events = Vec::new();
        match message {
            ClientMessage::Update { position } => {
                self.players.get_mut(&player_id).unwrap().position = position;
                events.push(Event::PlayerUpdated(self.players[&player_id].clone()));
            }
            ClientMessage::Event(event) => {
                self.handle(event.clone());
                events.push(event);
            }
        }
        events
    }
    #[must_use]
    pub fn tick(&mut self) -> Vec<Event> {
        vec![]
    }
    pub fn handle(&mut self, event: Event) {
        match event {
            Event::PlayerJoined(player) | Event::PlayerUpdated(player) => {
                let player_id = player.id;
                self.players.insert(player_id, player);
            }
            Event::PlayerLeft(player_id) => {
                self.players.remove(&player_id);
            }
            Event::TileBroken(position) => {
                self.tiles.remove(&position);
            }
            Event::TilePlaced(position, tile) => {
                if !self.tiles.contains_key(&position) {
                    self.tiles.insert(position, tile);
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Event {
    PlayerJoined(Player),
    PlayerUpdated(Player),
    PlayerLeft(Id),
    TileBroken(Vec2<i32>),
    TilePlaced(Vec2<i32>, Tile),
}
