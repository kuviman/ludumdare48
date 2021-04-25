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
    pub on_ground: bool,
    pub looks_right: bool,
    pub swing: Option<f32>,
    pub item: Option<Item>,
    pub money: usize,
    pub skin_tone: f64,
    pub stick: f64,
    pub hat_color: f64,
    pub beard: usize,
    pub ear: usize,
    pub eye: usize,
    pub hat: usize,
    pub mouth: usize,
    pub mustache: usize,
    pub nose: usize,
    pub name: String,
}

impl Player {
    pub const RANGE: f32 = 1.5;
    pub const SPEED: f32 = 3.0;
    pub const JUMP_SPEED: f32 = 4.0;
    pub const JUMP_TIME: f32 = 0.3;
    pub const SWING_SPEED: f32 = 2.0;
    pub fn new(id_gen: &mut IdGen) -> Self {
        Self {
            id: id_gen.gen(),
            position: vec2(0.0, 0.0),
            target_velocity: vec2(0.0, 0.0),
            size: vec2(0.5, 0.5),
            jump_timer: 0.0,
            on_ground: false,
            looks_right: true,
            swing: None,
            item: None,
            money: 0,
            skin_tone: global_rng().gen_range(0.0..1.0),
            stick: global_rng().gen_range(0.0..1.0),
            hat_color: global_rng().gen_range(0.0..1.0),
            beard: global_rng().gen_range(0..5),
            ear: global_rng().gen_range(0..4),
            eye: global_rng().gen_range(0..4),
            hat: global_rng().gen_range(0..5),
            mouth: global_rng().gen_range(0..4),
            mustache: global_rng().gen_range(0..5),
            nose: global_rng().gen_range(0..4),
            name: String::new(),
        }
    }
    pub fn matrix(&self) -> Mat4<f32> {
        let mut matrix = Mat4::translate(self.position.extend(0.0))
            * Mat4::scale(vec3(self.size.x, self.size.y, 1.0));
        if self.looks_right {
            matrix *= Mat4::scale(vec3(-1.0, 1.0, 1.0)) * Mat4::translate(vec3(-1.0, 0.0, 0.0));
        }
        matrix
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
        if self.collide(tiles, false, initial_position) {
            self.position.x = initial_position.x;
        }
        self.position.y += delta_position.y;
        self.on_ground = false;
        let on_platform = self.position.y.floor() == -1.0 && initial_position.y.floor() == 0.0;
        if self.collide(
            tiles,
            self.position.y < initial_position.y && self.target_velocity.y >= 0.0,
            initial_position,
        ) || on_platform && self.target_velocity.y >= 0.0
        {
            if self.position.y < initial_position.y {
                self.jump_timer = Self::JUMP_TIME;
                self.on_ground = true;
            } else {
                self.jump_timer = 0.0;
            }
            self.position.y = initial_position.y;
        }
        if self.collide(tiles, true, initial_position) {
            self.on_ground = true;
        }
        if self.position.x < initial_position.x {
            self.looks_right = false;
        }
        if self.position.x > initial_position.x {
            self.looks_right = true;
        }
        if let Some(swing) = &mut self.swing {
            *swing += delta_time * Self::SWING_SPEED;
        }
    }
    fn collide(
        &self,
        tiles: &TileMap,
        consider_climbable: bool,
        initial_position: Vec2<f32>,
    ) -> bool {
        for position in self.tiles() {
            if let Some(tile) = tiles.get(&position) {
                if !tile.can_move_through()
                    && !AABB::pos_size(
                        position.map(|x| x as f32) - self.size,
                        vec2(1.0, 1.0) + self.size,
                    )
                    .contains(initial_position)
                {
                    return true;
                }
                if consider_climbable && tile.can_climb() {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Tile {
    Stone,
    Ladder,
    Block,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum ItemType {
    Block,
    Ladder,
    Chest,
}

impl ItemType {
    pub fn random() -> Self {
        match global_rng().gen_range(0..3) {
            0 => Self::Block,
            1 => Self::Ladder,
            2 => Self::Chest,
            _ => unreachable!(),
        }
    }
    pub fn placed(&self) -> Option<Tile> {
        match self {
            Self::Block => Some(Tile::Block),
            Self::Chest => None,
            Self::Ladder => Some(Tile::Ladder),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Id,
    pub position: Vec2<f32>,
    pub item_type: ItemType,
}

impl Item {
    pub const SIZE: f32 = 0.5;
    pub fn new(id_gen: &mut IdGen, position: Vec2<f32>, item_type: ItemType) -> Self {
        Self {
            id: id_gen.gen(),
            position,
            item_type,
        }
    }
}

impl Tile {
    pub fn can_move_through(&self) -> bool {
        match self {
            Self::Stone => false,
            Self::Ladder => true,
            Self::Block => false,
        }
    }
    pub fn can_climb(&self) -> bool {
        match self {
            Self::Ladder => true,
            _ => false,
        }
    }
    pub fn transparent(&self) -> bool {
        match self {
            Self::Ladder => true,
            Self::Stone => false,
            Self::Block => false,
        }
    }
    pub fn need_border(&self) -> bool {
        match self {
            Self::Stone => true,
            Self::Ladder => false,
            Self::Block => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShopType {
    House,
    Train,
    Passport,
    Sell {
        require_item: ItemType,
        give_item: Option<ItemType>,
        needs_coin: bool,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shop {
    pub position: f32,
    pub shop_type: ShopType,
}

impl Shop {
    pub fn help(&self) -> &str {
        match &self.shop_type {
            ShopType::House => "Press E to customize yourself",
            ShopType::Passport => "Press E to change your name",
            ShopType::Sell { .. } => "Press E to perform the deal",
            ShopType::Train => "Press E to travel to the other world",
        }
    }
}

pub type TileMap = HashMap<Vec2<i32>, Tile>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    id_gen: IdGen,
    pub ticks_per_second: f64,
    pub players: HashMap<Id, Player>,
    pub items: HashMap<Id, Item>,
    pub tiles: TileMap,
    pub shops: Vec<Shop>,
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
            items: default(),
            shops: vec![
                Shop {
                    position: 2.0,
                    shop_type: ShopType::Sell {
                        require_item: ItemType::Chest,
                        give_item: None,
                        needs_coin: false,
                    },
                },
                Shop {
                    position: 6.0,
                    shop_type: ShopType::Sell {
                        require_item: ItemType::Block,
                        give_item: Some(ItemType::Ladder),
                        needs_coin: true,
                    },
                },
                Shop {
                    position: -2.0,
                    shop_type: ShopType::House,
                },
                Shop {
                    position: -6.0,
                    shop_type: ShopType::Passport,
                },
                Shop {
                    position: -10.0,
                    shop_type: ShopType::Train,
                },
            ],
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
            ClientMessage::Event(event) => {
                self.handle_impl(event.clone(), Some(&mut events));
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
        self.handle_impl(event, None);
    }
    pub fn handle_impl(&mut self, event: Event, events: Option<&mut Vec<Event>>) {
        match event {
            Event::PlayerJoined(player) | Event::PlayerUpdated(player) => {
                let player_id = player.id;
                self.players.insert(player_id, player);
            }
            Event::PlayerLeft(player_id) => {
                self.players.remove(&player_id);
            }
            Event::TileBroken(position) => {
                if let Some(tile) = self.tiles.remove(&position) {
                    if let Some(events) = events {
                        let event = Event::ItemAdded(Item::new(
                            &mut self.id_gen,
                            position.map(|x| x as f32)
                                + vec2(global_rng().gen_range(0.0..1.0), 0.0),
                            ItemType::Block,
                        ));
                        events.push(event.clone());
                        self.handle_impl(event, None);
                        if global_rng().gen_bool(0.1) {
                            let event = Event::ItemAdded(Item::new(
                                &mut self.id_gen,
                                position.map(|x| x as f32)
                                    + vec2(global_rng().gen_range(0.0..1.0), 0.0),
                                ItemType::Chest,
                            ));
                            events.push(event.clone());
                            self.handle_impl(event, None);
                        }
                    }
                }
            }
            Event::TilePlaced(position, tile) => {
                if !self.tiles.contains_key(&position) {
                    self.tiles.insert(position, tile);
                }
            }
            Event::ItemAdded(item) => {
                let item_id = item.id;
                self.items.insert(item_id, item);
            }
            Event::ItemRemoved(id) => {
                self.items.remove(&id);
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
    ItemAdded(Item),
    ItemRemoved(Id),
}
