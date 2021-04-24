use super::*;

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    camera: Camera,
    renderer: Renderer,
    model: Model,
    player: Player,
    connection: Connection,
    left_click: Option<Vec2<f32>>,
    right_click: Option<Vec2<f32>>,
    transition: Option<geng::Transition>,
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
            model: welcome.model,
            connection,
            left_click: None,
            right_click: None,
            transition: None,
        }
    }
    fn draw_player(&self, framebuffer: &mut ugli::Framebuffer, player: &Player) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(player.position.extend(0.0)) * Mat4::scale_uniform(0.5),
            &self.assets.player,
        );
    }
    fn draw_tile(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<i32>,
        texture: &ugli::Texture,
    ) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(position.map(|x| x as f32).extend(0.0)),
            &texture,
        );
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::rgb(0.8, 0.8, 1.0)), None);
        const RADIUS: i32 = 10;
        for x in self.player.position.x as i32 - RADIUS..=self.player.position.x as i32 + RADIUS {
            for y in self.player.position.y as i32 - RADIUS..=self.player.position.y as i32 + RADIUS
            {
                let position = vec2(x, y);
                if let Some(tile) = self.model.tiles.get(&position) {
                    let texture = match tile {
                        Tile::Stone => &self.assets.stone,
                    };
                    self.draw_tile(framebuffer, position, texture);
                }
            }
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
        let mut messages_to_send = Vec::new();
        if !messages.is_empty() {
            messages_to_send.push(ClientMessage::Update {
                position: self.player.position,
            });
            if let Some(position) = self.left_click {
                messages_to_send.push(ClientMessage::Event(Event::TileBroken(
                    position.map(|x| x.floor() as i32),
                )));
            }
            if let Some(position) = self.right_click {
                messages_to_send.push(ClientMessage::Event(Event::TilePlaced(
                    position.map(|x| x.floor() as i32),
                    Tile::Stone,
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
        let mut velocity = vec2(0.0, 0.0);
        if self.geng.window().is_key_pressed(geng::Key::A) {
            velocity.x -= 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::D) {
            velocity.x += 1.0;
        }
        // velocity *= 100.0;
        self.player.position += velocity * delta_time;
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
