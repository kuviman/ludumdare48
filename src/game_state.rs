use super::*;

struct PlayerState {
    step_animation: f32,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            step_animation: 0.0,
        }
    }
    pub fn update(&mut self, player: &Player, delta_time: f32) {
        self.step_animation += player.target_velocity.len() * delta_time;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    camera: Camera,
    renderer: Renderer,
    model: Model,
    player: Player,
    players: HashMap<Id, PlayerState>,
    connection: Connection,
    left_click: Option<Vec2<f32>>,
    right_click: Option<Vec2<f32>>,
    transition: Option<geng::Transition>,
    to_send: Vec<ClientMessage>,
    noise: noise::OpenSimplex,
}

impl GameState {
    pub fn new(
        geng: &Rc<Geng>,
        assets: &Rc<Assets>,
        welcome: WelcomeMessage,
        connection: Connection,
    ) -> Self {
        let player = welcome.model.players[&welcome.player_id].clone();
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera::new(10.0),
            renderer: Renderer::new(geng),
            player,
            players: HashMap::new(),
            model: welcome.model,
            connection,
            left_click: None,
            right_click: None,
            transition: None,
            to_send: Vec::new(),
            noise: noise::OpenSimplex::new(),
        }
    }
    fn draw_player_part(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        player: &Player,
        texture: &ugli::Texture,
        position: Vec2<f32>,
        flip_x: bool,
        rotation: f32,
        color: Color<f32>,
    ) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            player.matrix()
                * Mat4::translate(vec3(-1.0, -1.0, 0.0) + position.extend(0.0))
                * Mat4::scale_uniform(3.0)
                * Mat4::translate(vec3(0.5, 0.5, 0.0))
                * Mat4::rotate_z(rotation)
                * Mat4::scale(vec3(if flip_x { -1.0 } else { 1.0 }, 1.0, 1.0))
                * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
            texture,
            color,
        );
    }
    fn draw_item(&self, framebuffer: &mut ugli::Framebuffer, item: &Item) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(item.position.extend(0.0))
                * Mat4::scale_uniform(Item::SIZE)
                * Mat4::translate(vec3(-0.5, 0.0, 0.0)),
            match item.item_type {
                ItemType::Block => &self.assets.block_item,
                ItemType::Chest => &self.assets.chest,
                ItemType::Ladder => &self.assets.ladder_item,
            },
            Color::WHITE,
        );
    }
    fn draw_player(&self, framebuffer: &mut ugli::Framebuffer, player: &Player) {
        let state = if let Some(state) = self.players.get(&player.id) {
            state
        } else {
            return;
        };
        let leg_arg = state.step_animation * f32::PI * 2.0 * 5.0;
        let mut leg_amp = player.target_velocity.len().min(1.0) * 0.1;
        let mut leg_offset = 0.0;
        if !player.on_ground {
            leg_amp = 0.0;
            leg_offset = -0.1;
        }
        let mut pick_position = vec2(0.0, 0.0);
        let mut pick_rotation = f32::PI / 4.0;
        if let Some(swing) = player.swing {
            pick_rotation = swing * f32::PI * 2.0 + f32::PI;
            pick_position = Vec2::rotated(vec2(1.0, 0.0), pick_rotation);
        }
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.stick,
            pick_position,
            false,
            pick_rotation - f32::PI / 4.0,
            Color::WHITE,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.pick_head,
            pick_position,
            false,
            pick_rotation - f32::PI / 4.0,
            Color::WHITE,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.leg,
            vec2(
                0.0,
                (leg_arg + f32::PI).sin().max(0.0) * leg_amp + leg_offset,
            ),
            true,
            0.0,
            Color::WHITE,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.body,
            vec2(0.0, 0.0),
            false,
            0.0,
            Color::WHITE,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.leg,
            vec2(0.0, leg_arg.sin().max(0.0) * leg_amp + leg_offset),
            false,
            0.0,
            Color::WHITE,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.eyes,
            vec2(0.0, 0.0),
            false,
            0.0,
            Color::WHITE,
        );
    }
    fn draw_tile(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<i32>,
        texture: &ugli::Texture,
        color: Color<f32>,
    ) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(position.map(|x| x as f32).extend(0.0)),
            &texture,
            color,
        );
    }
    fn draw_random_tile(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<i32>,
        textures: &[ugli::Texture],
        color: Color<f32>,
        noise_offset: f64,
    ) {
        use noise::NoiseFn;
        let noise = self
            .noise
            .get([position.x as f64 + noise_offset, position.y as f64]);
        let noise = (noise / 0.544 + 1.0) / 2.0;
        let index = clamp(
            (noise * textures.len() as f64) as i32,
            0..=textures.len() as i32 - 1,
        ) as usize;
        self.draw_tile(framebuffer, position, &textures[index], color);
    }
    fn update_player(&mut self, delta_time: f32) {
        self.player.target_velocity = vec2(0.0, 0.0);
        if self.geng.window().is_key_pressed(geng::Key::A) {
            self.player.target_velocity.x -= 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::D) {
            self.player.target_velocity.x += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::W) {
            self.player.target_velocity.y += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::S) {
            self.player.target_velocity.y -= 1.0;
        }
        self.player.update(&self.model.tiles, delta_time);
        if let Some(position) = self.left_click {
            let position = position.map(|x| x.floor() as i32);
            match self.player.swing {
                None => self.player.swing = Some(0.0),
                Some(swing) if swing > 1.0 => {
                    self.to_send
                        .push(ClientMessage::Event(Event::TileBroken(position)));
                    self.player.swing = Some(0.0);
                }
                _ => {}
            }
        } else {
            self.player.swing = None;
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.camera.center = self.player.position;
        ugli::clear(framebuffer, Some(Color::rgb(0.8, 0.8, 1.0)), None);
        const RADIUS: i32 = 10;
        for x in self.player.position.x as i32 - RADIUS..=self.player.position.x as i32 + RADIUS {
            self.draw_tile(framebuffer, vec2(x, 0), &self.assets.grass, Color::WHITE);
        }
        for x in self.player.position.x as i32 - RADIUS..=self.player.position.x as i32 + RADIUS {
            for y in self.player.position.y as i32 - RADIUS..=self.player.position.y as i32 + RADIUS
            {
                let position = vec2(x, y);
                let mut draw_background = true;
                let current_tile = self.model.tiles.get(&position);
                if let Some(tile) = current_tile {
                    if !tile.transparent() {
                        draw_background = false;
                    }
                }
                if y < 0 && draw_background {
                    self.draw_random_tile(
                        framebuffer,
                        position,
                        &self.assets.stone,
                        Color::GRAY,
                        100.0,
                    );
                }
                if let Some(tile) = current_tile {
                    self.draw_random_tile(
                        framebuffer,
                        position,
                        match tile {
                            Tile::Stone => &self.assets.stone,
                            Tile::Ladder => &self.assets.ladder,
                            Tile::Block => &self.assets.block,
                        },
                        if *tile == Tile::Block {
                            Color::rgb(0.8, 0.8, 0.8)
                        } else {
                            Color::WHITE
                        },
                        0.0,
                    );
                }
            }
        }
        for x in self.player.position.x as i32 - RADIUS..=self.player.position.x as i32 + RADIUS {
            for y in self.player.position.y as i32 - RADIUS..=self.player.position.y as i32 + RADIUS
            {
                let position = vec2(x, y);
                let current_tile = self.model.tiles.get(&position);
                let right_tile = self.model.tiles.get(&(position + vec2(1, 0)));
                let top_tile = self.model.tiles.get(&(position + vec2(0, 1)));
                let current_need_border =
                    current_tile.map(|tile| tile.need_border()).unwrap_or(false);
                let right_need_border = right_tile.map(|tile| tile.need_border()).unwrap_or(false);
                let top_need_border = top_tile.map(|tile| tile.need_border()).unwrap_or(false);
                if current_tile != right_tile && (current_need_border || right_need_border) {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(position.x as f32 + 0.5, position.y as f32, 0.0)),
                        &self.assets.border,
                        Color::BLACK,
                    );
                }
                if current_tile != top_tile && (current_need_border || top_need_border) {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(
                            position.x as f32 + 1.0,
                            position.y as f32 + 0.5,
                            0.0,
                        )) * Mat4::rotate_z(f32::PI / 2.0),
                        &self.assets.border,
                        Color::BLACK,
                    );
                }
            }
        }
        for item in self.model.items.values() {
            self.draw_item(framebuffer, item);
        }
        self.draw_player(framebuffer, &self.player);
        for player in self.model.players.values() {
            if player.id == self.player.id {
                continue;
            }
            self.draw_player(framebuffer, player);
        }
        if self
            .geng
            .window()
            .is_button_pressed(geng::MouseButton::Left)
        {
            self.left_click = Some(self.camera.screen_to_world(
                framebuffer,
                self.geng.window().mouse_pos().map(|x| x as f32),
            ));
        } else {
            self.left_click = None;
        }
        if self
            .geng
            .window()
            .is_button_pressed(geng::MouseButton::Right)
        {
            self.right_click = Some(self.camera.screen_to_world(
                framebuffer,
                self.geng.window().mouse_pos().map(|x| x as f32),
            ));
        } else {
            self.right_click = None;
        }
    }
    fn update(&mut self, delta_time: f64) {
        let mut messages = Vec::new();
        match &mut self.connection {
            Connection::Remote(connection) => messages.extend(connection.new_messages()),
            Connection::Local { next_tick, model } => {
                *next_tick -= delta_time;
                while *next_tick <= 0.0 {
                    messages.push(ServerMessage::Update(model.tick()));
                    *next_tick += 1.0 / model.ticks_per_second;
                }
            }
        }
        let mut messages_to_send = mem::replace(&mut self.to_send, Vec::new());
        if !messages.is_empty() {
            messages_to_send.push(ClientMessage::Event(Event::PlayerUpdated(
                self.player.clone(),
            )));
            if let Some(position) = self.right_click {
                messages_to_send.push(ClientMessage::Event(Event::TilePlaced(
                    position.map(|x| x.floor() as i32),
                    Tile::Block,
                )));
            }
        }
        for message in messages_to_send {
            match &mut self.connection {
                Connection::Remote(connection) => connection.send(message),
                Connection::Local {
                    next_tick: _,
                    model,
                } => {
                    messages.push(ServerMessage::Update(
                        model.handle_message(self.player.id, message),
                    ));
                }
            }
        }
        for message in messages {
            match message {
                ServerMessage::Update(events) => {
                    for event in events {
                        self.model.handle(event);
                    }
                }
                _ => unreachable!(),
            }
        }
        let delta_time = delta_time as f32;
        self.update_player(delta_time);

        for player in self.model.players.values() {
            if player.id == self.player.id {
                continue;
            }
            self.players
                .entry(player.id)
                .or_default()
                .update(player, delta_time);
        }
        self.players
            .entry(self.player.id)
            .or_default()
            .update(&self.player, delta_time);
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key, .. } => match key {
                geng::Key::Escape => {
                    self.transition = Some(geng::Transition::Pop);
                }
                _ => {}
            },
            _ => {}
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }
}
