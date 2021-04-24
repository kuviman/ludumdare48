use super::*;

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    camera: Camera,
    renderer: Renderer,
    model: Model,
    player: Player,
    connection: Connection,
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
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::rgb(0.8, 0.8, 1.0)), None);
        self.draw_player(framebuffer, &self.player);
        for player in self.model.players.values() {
            if player.id == self.player.id {
                continue;
            }
            self.draw_player(framebuffer, player);
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
        if !messages.is_empty() {
            let message = ClientMessage::Update {
                position: self.player.position,
            };
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
