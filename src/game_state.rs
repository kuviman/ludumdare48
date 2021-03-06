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
struct UiState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    volume_slider: geng::ui::Slider,
    volume: f64,
    skin_tone: f64,
    skin_tone_slider: geng::ui::Slider,
    stick: f64,
    stick_slider: geng::ui::Slider,
    hat_color: f64,
    hat_color_slider: geng::ui::Slider,
    beard: usize,
    beard_slider: geng::ui::Slider,
    ear: usize,
    ear_slider: geng::ui::Slider,
    eye: usize,
    eye_slider: geng::ui::Slider,
    hat: usize,
    hat_slider: geng::ui::Slider,
    mouth: usize,
    mouth_slider: geng::ui::Slider,
    mustache: usize,
    mustache_slider: geng::ui::Slider,
    nose: usize,
    nose_slider: geng::ui::Slider,
    customize_character: bool,
    changing_name: bool,
    leaderboard: bool,
}

const SOUND_RANGE: f32 = 5.0;

impl UiState {
    fn locked(&self) -> bool {
        self.changing_name || self.customize_character || self.leaderboard
    }
    fn new(geng: &Rc<Geng>, assets: &Rc<Assets>, player: &Player) -> Self {
        let mut ui_theme = geng::ui::Theme::default(geng);
        ui_theme.usable_color = Color::GRAY;
        ui_theme.text_color = Color::BLACK;
        ui_theme.hover_color = Color::BLUE;
        let ui_theme = Rc::new(ui_theme);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            volume_slider: geng::ui::Slider::new(&ui_theme),
            volume: 0.5,
            skin_tone: player.skin_tone,
            skin_tone_slider: geng::ui::Slider::new(&ui_theme),
            stick: player.stick,
            stick_slider: geng::ui::Slider::new(&ui_theme),
            hat_color: player.hat_color,
            hat_color_slider: geng::ui::Slider::new(&ui_theme),
            beard: player.beard,
            beard_slider: geng::ui::Slider::new(&ui_theme),
            ear: player.ear,
            ear_slider: geng::ui::Slider::new(&ui_theme),
            eye: player.eye,
            eye_slider: geng::ui::Slider::new(&ui_theme),
            hat: player.hat,
            hat_slider: geng::ui::Slider::new(&ui_theme),
            mouth: player.mouth,
            mouth_slider: geng::ui::Slider::new(&ui_theme),
            mustache: player.mustache,
            mustache_slider: geng::ui::Slider::new(&ui_theme),
            nose: player.nose,
            nose_slider: geng::ui::Slider::new(&ui_theme),
            customize_character: false,
            changing_name: false,
            leaderboard: false,
        }
    }
    fn ui<'a>(&'a mut self, model: &'a Model, local: bool) -> impl geng::ui::Widget + 'a {
        use geng::ui;
        use geng::ui::*;
        let font: &Rc<geng::Font> = &self.assets.font;
        let volume = &mut self.volume;
        let mut stack = ui::stack![ui::row![
            geng::ui::Text::new("volume", font, 30.0, Color::BLACK).padding_right(30.0),
            self.volume_slider
                .ui(
                    *volume,
                    0.0..=1.0,
                    Box::new(move |new_value| *volume = new_value)
                )
                .fixed_size(vec2(100.0, 30.0))
        ]
        .padding_right(50.0)
        .padding_bottom(50.0)
        .align(vec2(1.0, 0.0))];
        if self.leaderboard {
            if local {
                stack.push(Box::new(
                    geng::ui::stack![
                        geng::ui::ColorBox::new(&self.geng, Color::rgba(1.0, 1.0, 1.0, 0.7)),
                        geng::ui::Text::new(
                            "This is single player world, use the train to play with other people",
                            font,
                            50.0,
                            Color::BLACK
                        )
                        .uniform_padding(30.0),
                    ]
                    .align(vec2(0.5, 0.5)),
                ));
            } else {
                let mut column = ui::column(vec![]);
                let mut players: Vec<_> = model
                    .leaderboard
                    .values()
                    .map(|player| (player.money, &player.name))
                    .collect();
                players.sort();
                players.reverse();
                column.push(Box::new(
                    ui::row![
                        geng::ui::Text::new("Rank", font, 30.0, Color::BLACK)
                            .align(vec2(1.0, 0.0))
                            .uniform_padding(10.0)
                            .fixed_size(vec2(100.0, 50.0)),
                        geng::ui::Text::new("Name", font, 30.0, Color::BLACK)
                            .align(vec2(0.0, 0.0))
                            .uniform_padding(10.0)
                            .fixed_size(vec2(400.0, 50.0)),
                        geng::ui::Text::new("Score", font, 30.0, Color::BLACK)
                            .align(vec2(0.0, 0.0))
                            .uniform_padding(10.0)
                            .fixed_size(vec2(100.0, 50.0))
                    ]
                    .padding_bottom(50.0),
                ));
                for (index, (score, player)) in players.into_iter().take(10).enumerate() {
                    column.push(Box::new(ui::stack![
                        geng::ui::ColorBox::new(
                            &self.geng,
                            Color::rgba(1.0, 1.0, 1.0, if index % 2 == 0 { 0.9 } else { 0.0 }),
                        ),
                        ui::row![
                            geng::ui::Text::new((index + 1).to_string(), font, 30.0, Color::BLACK)
                                .align(vec2(1.0, 0.0))
                                .uniform_padding(10.0)
                                .fixed_size(vec2(60.0, 50.0)),
                            geng::ui::Text::new(player, font, 30.0, Color::BLACK)
                                .align(vec2(0.0, 0.0))
                                .uniform_padding(10.0)
                                .fixed_size(vec2(400.0, 50.0)),
                            geng::ui::Text::new(score.to_string(), font, 30.0, Color::BLACK)
                                .align(vec2(0.0, 0.0))
                                .uniform_padding(10.0)
                                .fixed_size(vec2(100.0, 50.0))
                        ],
                    ]));
                }
                stack.push(Box::new(
                    geng::ui::stack![
                        geng::ui::ColorBox::new(&self.geng, Color::rgba(1.0, 1.0, 1.0, 0.7)),
                        column.uniform_padding(30.0),
                    ]
                    .align(vec2(0.5, 0.5)),
                ));
            }
        }
        if self.customize_character {
            let mut column = ui::column(vec![]);
            let skin_tone = &mut self.skin_tone;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("skin tone", font, 50.0, Color::BLACK).padding_right(24.0),
                self.skin_tone_slider
                    .ui(
                        *skin_tone,
                        0.0..=1.0,
                        Box::new(move |new_value| *skin_tone = new_value)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let stick = &mut self.stick;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("pickaxe color", font, 50.0, Color::BLACK).padding_right(24.0),
                self.stick_slider
                    .ui(
                        *stick,
                        0.0..=1.0,
                        Box::new(move |new_value| *stick = new_value)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let eye = &mut self.eye;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("eye", font, 50.0, Color::BLACK).padding_right(50.0),
                self.eye_slider
                    .ui(
                        *eye as f64,
                        0.0..=3.0,
                        Box::new(move |new_value| *eye = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let ear = &mut self.ear;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("ear", font, 50.0, Color::BLACK).padding_right(50.0),
                self.ear_slider
                    .ui(
                        *ear as f64,
                        0.0..=3.0,
                        Box::new(move |new_value| *ear = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let mouth = &mut self.mouth;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("mouth", font, 50.0, Color::BLACK).padding_right(50.0),
                self.mouth_slider
                    .ui(
                        *mouth as f64,
                        0.0..=3.0,
                        Box::new(move |new_value| *mouth = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let nose = &mut self.nose;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("nose", font, 50.0, Color::BLACK).padding_right(50.0),
                self.nose_slider
                    .ui(
                        *nose as f64,
                        0.0..=3.0,
                        Box::new(move |new_value| *nose = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let mustache = &mut self.mustache;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("mustache", font, 50.0, Color::BLACK).padding_right(50.0),
                self.mustache_slider
                    .ui(
                        *mustache as f64,
                        0.0..=4.0,
                        Box::new(move |new_value| *mustache = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let beard = &mut self.beard;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("beard", font, 50.0, Color::BLACK).padding_right(50.0),
                self.beard_slider
                    .ui(
                        *beard as f64,
                        0.0..=4.0,
                        Box::new(move |new_value| *beard = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let hat = &mut self.hat;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("hat", font, 50.0, Color::BLACK).padding_right(50.0),
                self.hat_slider
                    .ui(
                        *hat as f64,
                        0.0..=4.0,
                        Box::new(move |new_value| *hat = new_value.round() as usize)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            let hat_color = &mut self.hat_color;
            column.push(Box::new(ui::row![
                geng::ui::Text::new("hat color", font, 50.0, Color::BLACK).padding_right(24.0),
                self.hat_color_slider
                    .ui(
                        *hat_color,
                        0.0..=1.0,
                        Box::new(move |new_value| *hat_color = new_value)
                    )
                    .fixed_size(vec2(300.0, 50.0))
            ]));
            stack.push(Box::new(
                geng::ui::stack![
                    geng::ui::ColorBox::new(&self.geng, Color::rgba(1.0, 1.0, 1.0, 0.7)),
                    column.uniform_padding(50.0),
                ]
                .align(vec2(0.0, 1.0)),
            ));
        }
        stack
    }
    fn update_player(&self, player: &mut Player) {
        player.skin_tone = self.skin_tone;
        player.stick = self.stick;
        player.hat_color = self.hat_color;
        player.beard = self.beard;
        player.ear = self.ear;
        player.eye = self.eye;
        player.hat = self.hat;
        player.mouth = self.mouth;
        player.mustache = self.mustache;
        player.nose = self.nose;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

const HELPS: &[&str] = &[
    "Use WASD/Arrows to move around",
    "Use Left Mouse Button to swing your pickaxe",
    "Use E to pick up items",
    "Use Q to drop items",
    "Use Right Mouse Button to place a block",
    "Dig deeper and deeper and you'll get more and more valuable treasure",
    "By the way, music is bad on purpose",
    "But at least ther IS music, right?",
];

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    opt: Rc<Opt>,
    camera: Camera,
    renderer: Renderer,
    model: Model,
    player: Player,
    players: HashMap<Id, PlayerState>,
    connection: Connection,
    left_click: Option<Vec2<f32>>,
    transition: Option<geng::Transition>,
    to_send: Vec<ClientMessage>,
    noise: noise::OpenSimplex,
    framebuffer_size: Vec2<f32>,
    ui_state: UiState,
    ui_controller: geng::ui::Controller,
    current_help: usize,
    music: Option<geng::SoundEffect>,
}

impl Drop for GameState {
    fn drop(&mut self) {
        if let Connection::Remote(connection) = &mut self.connection {
            connection.send(ClientMessage::Event(Event::PlayerLeft(self.player.id)));
        }
    }
}

impl GameState {
    pub fn new(
        geng: &Rc<Geng>,
        assets: &Rc<Assets>,
        opt: &Rc<Opt>,
        player: Option<Player>,
        welcome: WelcomeMessage,
        connection: Connection,
    ) -> Self {
        let player = match player {
            Some(mut player) => {
                player.id = welcome.player_id;
                player
            }
            None => welcome.model.players[&welcome.player_id].clone(),
        };
        let ui_state = UiState::new(geng, assets, &player);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            opt: opt.clone(),
            camera: Camera::new(10.0),
            renderer: Renderer::new(geng),
            player,
            players: HashMap::new(),
            model: welcome.model,
            connection,
            left_click: None,
            transition: None,
            to_send: Vec::new(),
            noise: noise::OpenSimplex::new(),
            framebuffer_size: vec2(1.0, 1.0),
            ui_state,
            ui_controller: geng::ui::Controller::new(),
            current_help: HELPS.len(),
            music: None,
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
        scale: f32,
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
                * Mat4::scale(vec3(if flip_x { -1.0 } else { 1.0 }, 1.0, 1.0) * scale)
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
            self.assets.item_texture(item.item_type),
            item.item_type.color(),
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
        if let Some(swing) = player.swing.or(
            if self.ui_state.customize_character && self.player.id == player.id {
                Some(-1.0 / 8.0)
            } else {
                None
            },
        ) {
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
            1.0,
            hsv(player.stick as f32, 0.5, 0.7),
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.pick_head,
            pick_position,
            false,
            pick_rotation - f32::PI / 4.0,
            1.0,
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
            1.0,
            Color::GRAY,
        );
        let skin_color = hsv(
            6.0 / 255.0,
            80.0 / 255.0,
            (50.0 + (255.0 - 50.0) * player.skin_tone as f32) / 255.0,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.body,
            vec2(0.0, 0.0),
            false,
            0.0,
            1.0,
            skin_color,
        );
        self.draw_player_part(
            framebuffer,
            player,
            &self.assets.leg,
            vec2(0.0, leg_arg.sin().max(0.0) * leg_amp + leg_offset),
            false,
            0.0,
            1.0,
            Color::GRAY,
        );
        if let Some(texture) = self.assets.eye.get(player.eye) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                Color::WHITE,
            );
        }
        if let Some(texture) = self.assets.mouth.get(player.mouth) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                Color::WHITE,
            );
        }
        if let Some(texture) = self.assets.beard.get(player.beard) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                Color::WHITE,
            );
        }
        if let Some(texture) = self.assets.hat.get(player.hat) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                hsv(player.hat_color as f32, 0.5, 0.7),
            );
        }
        if let Some(texture) = self.assets.ear.get(player.ear) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                skin_color,
            );
        }
        if let Some(texture) = self.assets.mustache.get(player.mustache) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                Color::WHITE,
            );
        }
        if let Some(texture) = self.assets.nose.get(player.nose) {
            self.draw_player_part(
                framebuffer,
                player,
                texture,
                vec2(0.0, 0.0),
                false,
                0.0,
                1.0,
                skin_color,
            );
        }
        if let Some(item) = &player.item {
            self.draw_player_part(
                framebuffer,
                player,
                self.assets.item_texture(item.item_type),
                vec2(0.0, 1.0),
                false,
                0.0,
                0.3,
                item.item_type.color(),
            )
        }
        let changing_name = self.ui_state.changing_name && player.id == self.player.id;
        if !player.name.is_empty() || changing_name {
            let mut text = player.name.clone();
            if changing_name {
                text.push('_');
            }
            let pos = self.camera.world_to_screen(
                framebuffer.size().map(|x| x as f32),
                player.position + vec2(player.size.x / 2.0, player.size.y * 2.0),
            );
            let font = &self.assets.font;
            let size = framebuffer.size().y as f32 / self.camera.fov / 3.0;
            let text_width = font.measure(&text, size).width();
            self.geng.draw_2d().quad(
                framebuffer,
                AABB::pos_size(
                    pos - vec2(text_width / 2.0 + 5.0, 5.0),
                    vec2(text_width + 10.0, size + 10.0),
                ),
                Color::rgba(1.0, 1.0, 1.0, 0.7),
            );
            font.draw_aligned(framebuffer, &text, pos, 0.5, size, Color::BLACK);
        }
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
        self.ui_state.update_player(&mut self.player);
        self.player.target_velocity = vec2(0.0, 0.0);
        if !self.ui_state.locked() {
            if self.geng.window().is_key_pressed(geng::Key::A)
                || self.geng.window().is_key_pressed(geng::Key::Left)
            {
                self.player.target_velocity.x -= 1.0;
            }
            if self.geng.window().is_key_pressed(geng::Key::D)
                || self.geng.window().is_key_pressed(geng::Key::Right)
            {
                self.player.target_velocity.x += 1.0;
            }
            if self.geng.window().is_key_pressed(geng::Key::W)
                || self.geng.window().is_key_pressed(geng::Key::Up)
            {
                self.player.target_velocity.y += 1.0;
            }
            if self.geng.window().is_key_pressed(geng::Key::S)
                || self.geng.window().is_key_pressed(geng::Key::Down)
            {
                self.player.target_velocity.y -= 1.0;
            }
        }
        self.player.update(&self.model.tiles, delta_time);
        if let Some(position) = self.left_click {
            let position = position.map(|x| x.floor() as i32);
            match self.player.swing {
                None => self.player.swing = Some(0.0),
                Some(swing) if swing > 1.0 => {
                    if ((self.player.position + self.player.size / 2.0)
                        - position.map(|x| x as f32 + 0.5))
                    .len()
                        < Player::RANGE
                    {
                        self.to_send
                            .push(ClientMessage::Event(Event::TileBroken(position)));
                    }
                    self.player.swing = Some(0.0);
                }
                _ => {}
            }
        } else {
            self.player.swing = None;
        }
    }

    fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        if self.ui_state.locked() {
            self.camera.target_position = self.player.position;
            self.camera.target_fov = 3.0;
        } else {
            self.camera.target_position = self.player.position;
            self.camera.target_fov = 10.0;
        }
        ugli::clear(framebuffer, Some(Color::rgb(0.8, 0.8, 1.0)), None);
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(vec3(self.camera.center.x * 0.8 - 8.0, 0.0, 0.0))
                * Mat4::scale(vec3(16.0, 8.0, 1.0)),
            &self.assets.background,
            Color::rgba(1.0, 1.0, 1.0, 0.5),
        );
        const VIEW_RADIUS: i32 = 12;
        for shop in &self.model.shops {
            match shop.shop_type {
                ShopType::Sell {
                    needs_coin,
                    give_item,
                    require_item,
                } => {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0)) * Mat4::scale_uniform(2.0),
                        if needs_coin {
                            &self.assets.combine_shop
                        } else {
                            &self.assets.sell_shop
                        },
                        Color::WHITE,
                    );
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0) + vec3(0.5, 1.5, 0.0))
                            * Mat4::scale_uniform(0.5)
                            * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
                        self.assets.item_texture(require_item),
                        require_item.color(),
                    );
                    let give_texture = match give_item {
                        Some(item) => self.assets.item_texture(item),
                        None => &self.assets.coin,
                    };
                    if needs_coin {
                        self.renderer.draw(
                            framebuffer,
                            &self.camera,
                            Mat4::translate(vec3(shop.position, 0.0, 0.0) + vec3(1.5, 1.5, 0.0))
                                * Mat4::scale_uniform(0.5)
                                * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
                            &self.assets.coin,
                            Color::WHITE,
                        );
                        self.renderer.draw(
                            framebuffer,
                            &self.camera,
                            Mat4::translate(vec3(shop.position, 0.0, 0.0) + vec3(1.5, 0.5, 0.0))
                                * Mat4::scale_uniform(0.5)
                                * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
                            give_texture,
                            Color::WHITE,
                        );
                    } else {
                        self.renderer.draw(
                            framebuffer,
                            &self.camera,
                            Mat4::translate(vec3(shop.position, 0.0, 0.0) + vec3(1.5, 1.5, 0.0))
                                * Mat4::scale_uniform(0.5)
                                * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
                            give_texture,
                            give_item.map(|item| item.color()).unwrap_or(Color::WHITE),
                        );
                    }
                }
                ShopType::House => {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0)) * Mat4::scale_uniform(2.0),
                        &self.assets.house,
                        Color::WHITE,
                    );
                }
                ShopType::Train => {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0)) * Mat4::scale_uniform(2.0),
                        &self.assets.train,
                        Color::WHITE,
                    );
                }
                ShopType::Passport => {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0)) * Mat4::scale_uniform(2.0),
                        &self.assets.passport,
                        Color::WHITE,
                    );
                }
                ShopType::LeaderBoard => {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0)) * Mat4::scale_uniform(2.0),
                        &self.assets.leaderboard,
                        Color::WHITE,
                    );
                }
                ShopType::Info => {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(shop.position, 0.0, 0.0)) * Mat4::scale_uniform(2.0),
                        &self.assets.info,
                        Color::WHITE,
                    );
                }
            }
        }
        for x in self.player.position.x as i32 - VIEW_RADIUS
            ..=self.player.position.x as i32 + VIEW_RADIUS
        {
            self.draw_tile(framebuffer, vec2(x, 0), &self.assets.grass, Color::WHITE);
        }
        for x in self.player.position.x as i32 - VIEW_RADIUS
            ..=self.player.position.x as i32 + VIEW_RADIUS
        {
            for y in self.player.position.y as i32 - VIEW_RADIUS
                ..=self.player.position.y as i32 + VIEW_RADIUS
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
                        if y == -1 {
                            &self.assets.dirt
                        } else {
                            &self.assets.stone
                        },
                        Color::GRAY,
                        100.0,
                    );
                }
                if let Some(tile) = current_tile {
                    self.draw_random_tile(
                        framebuffer,
                        position,
                        self.assets.tile_textures(*tile),
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
        for x in self.player.position.x as i32 - VIEW_RADIUS
            ..=self.player.position.x as i32 + VIEW_RADIUS
        {
            for y in self.player.position.y as i32 - VIEW_RADIUS
                ..=self.player.position.y as i32 + VIEW_RADIUS
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
        {
            let position = self
                .camera
                .screen_to_world(
                    self.framebuffer_size,
                    self.geng.window().mouse_pos().map(|x| x as f32),
                )
                .map(|x| x.floor() as i32);
            if ((self.player.position + self.player.size / 2.0) - position.map(|x| x as f32 + 0.5))
                .len()
                < Player::RANGE
            {
                if self.model.tiles.get(&position).is_some() {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(
                            position.x as f32 + 0.5,
                            position.y as f32 + 0.5,
                            0.0,
                        )) * Mat4::rotate_z(f32::PI / 4.0)
                            * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
                        &self.assets.border,
                        Color::rgba(0.0, 0.0, 0.0, 0.5),
                    );
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(vec3(
                            position.x as f32 + 0.5,
                            position.y as f32 + 0.5,
                            0.0,
                        )) * Mat4::rotate_z(-f32::PI / 4.0)
                            * Mat4::translate(vec3(-0.5, -0.5, 0.0)),
                        &self.assets.border,
                        Color::rgba(0.0, 0.0, 0.0, 0.5),
                    );
                }
            }
        }
        if let Some(item) = &self.player.item {
            let position = self
                .camera
                .screen_to_world(
                    self.framebuffer_size,
                    self.geng.window().mouse_pos().map(|x| x as f32),
                )
                .map(|x| x.floor() as i32);
            if !self.model.tiles.contains_key(&position) {
                if let Some(tile) = item.item_type.placed() {
                    if ((self.player.position + self.player.size / 2.0)
                        - position.map(|x| x as f32 + 0.5))
                    .len()
                        < Player::RANGE
                    {
                        self.draw_random_tile(
                            framebuffer,
                            position,
                            self.assets.tile_textures(tile),
                            Color::rgba(1.0, 1.0, 1.0, 0.5),
                            0.0,
                        );
                    }
                }
            }
        }
        for item in self.model.items.values() {
            let delta_pos = item.position - self.player.position;
            let distance = delta_pos.x.abs().max(delta_pos.y.abs());
            if distance < VIEW_RADIUS as f32 {
                self.draw_item(framebuffer, item);
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
            && !self.ui_state.locked()
        {
            self.left_click = Some(self.camera.screen_to_world(
                framebuffer.size().map(|x| x as f32),
                self.geng.window().mouse_pos().map(|x| x as f32),
            ));
        } else {
            self.left_click = None;
        }
        let font = &self.assets.font;
        let text = self.player.money.to_string();
        self.geng.draw_2d().quad(
            framebuffer,
            AABB::pos_size(
                vec2(100.0, 60.0),
                vec2(font.measure(&text, 100.0).width() + 60.0, 80.0),
            ),
            Color::rgba(1.0, 1.0, 1.0, 0.7),
        );
        self.geng.draw_2d().textured_quad(
            framebuffer,
            AABB::pos_size(vec2(50.0, 50.0), vec2(100.0, 100.0)),
            &self.assets.coin,
            Color::WHITE,
        );
        font.draw(framebuffer, &text, vec2(150.0, 50.0), 100.0, Color::BLACK);
        if !self.ui_state.locked() {
            let shop = self.model.shops.iter().find(|shop| {
                AABB::pos_size(vec2(shop.position, 0.0) - self.player.size, vec2(2.0, 2.0))
                    .contains(self.player.position)
            });
            if let Some(shop) = shop {
                self.draw_text(
                    framebuffer,
                    vec2(shop.position + 1.0, 3.0),
                    40.0,
                    shop.help(),
                );
                if matches!(shop.shop_type, ShopType::Info) {
                    if let Some(text) = HELPS.get(self.current_help) {
                        self.draw_text(framebuffer, vec2(1.0 + shop.position, 2.5), 30.0, text);
                    }
                }
            }
        }
    }
    fn draw_text(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<f32>,
        size: f32,
        text: &str,
    ) {
        let font = &self.assets.font;
        let text_width = font.measure(text, size).width();
        let position = self
            .camera
            .world_to_screen(framebuffer.size().map(|x| x as f32), position);
        self.geng.draw_2d().quad(
            framebuffer,
            AABB::pos_size(
                position - vec2(text_width / 2.0 + 5.0, 5.0),
                vec2(text_width + 10.0, size + 10.0),
            ),
            Color::rgba(1.0, 1.0, 1.0, 0.7),
        );
        font.draw_aligned(framebuffer, text, position, 0.5, size, Color::BLACK);
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
        self.ui_controller.draw(
            &mut self.ui_state.ui(&self.model, self.connection.is_local()),
            framebuffer,
        );
    }
    fn update(&mut self, delta_time: f64) {
        if let Some(music) = &mut self.music {
            music.set_volume(self.ui_state.volume * 0.3);
        }
        self.camera.update(delta_time as f32);
        self.ui_controller.update(
            &mut self.ui_state.ui(&self.model, self.connection.is_local()),
            delta_time,
        );
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
                        match event {
                            Event::TilePlaced(position, ..) => {
                                if (position.map(|x| x as f32) - self.player.position).len()
                                    < SOUND_RANGE
                                {
                                    let mut effect = self.assets.place.effect();
                                    effect.set_volume(self.ui_state.volume);
                                    effect.play();
                                }
                            }
                            Event::TileBroken(position, ..)
                                if self.model.tiles.contains_key(&position) =>
                            {
                                if (position.map(|x| x as f32) - self.player.position).len()
                                    < SOUND_RANGE
                                {
                                    let mut effect = self.assets.dig.effect();
                                    effect.set_volume(self.ui_state.volume);
                                    effect.play();
                                }
                            }
                            _ => {}
                        }
                        self.model.handle(event);
                    }
                }
                _ => unreachable!(),
            }
        }
        let delta_time = delta_time as f32;
        self.update_player(delta_time);
        for player in self.model.players.values_mut() {
            player.update(&self.model.tiles, delta_time);
        }

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
        for item in self.model.items.values_mut() {
            let new_pos = vec2(item.position.x, item.position.y - delta_time * 5.0);
            if self
                .model
                .tiles
                .get(&new_pos.map(|x| x.floor() as i32))
                .is_none()
            {
                item.position = new_pos;
            }
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        self.ui_controller.handle_event(
            &mut self.ui_state.ui(&self.model, self.connection.is_local()),
            event.clone(),
        );
        if let geng::Event::KeyDown { key, .. } = event {
            let c = format!("{:?}", key);
            if c.len() == 1 && self.ui_state.changing_name {
                if self.player.name.len() < 20 {
                    self.player.name.push_str(&c);
                }
                return;
            }
            if key == geng::Key::Backspace && self.ui_state.changing_name {
                self.player.name.pop();
            }
        }
        match event {
            geng::Event::KeyDown { key, .. } => match key {
                geng::Key::Escape | geng::Key::Enter | geng::Key::E if self.ui_state.locked() => {
                    self.ui_state.customize_character = false;
                    self.ui_state.changing_name = false;
                    self.ui_state.leaderboard = false;
                }
                geng::Key::E => {
                    let shop = self.model.shops.iter().find(|shop| {
                        AABB::pos_size(vec2(shop.position, 0.0) - self.player.size, vec2(2.0, 2.0))
                            .contains(self.player.position)
                    });
                    if let Some(shop) = shop {
                        match shop.shop_type {
                            ShopType::Train => {
                                self.transition = Some(match self.connection {
                                    Connection::Local { .. } => {
                                        geng::Transition::Push(Box::new(ConnectingState::new(
                                            &self.geng,
                                            &self.assets,
                                            &self.opt,
                                            Some(self.player.clone()),
                                        )))
                                    }
                                    Connection::Remote(_) => geng::Transition::Pop,
                                });
                            }
                            ShopType::House => {
                                self.ui_state.customize_character = true;
                            }
                            ShopType::Passport => {
                                self.ui_state.changing_name = true;
                            }
                            ShopType::LeaderBoard => {
                                self.ui_state.leaderboard = true;
                            }
                            ShopType::Info => {
                                self.current_help += 1;
                                if self.current_help >= HELPS.len() {
                                    self.current_help = 0;
                                }
                            }
                            _ => {}
                        }
                    }
                    if let Some(item) = self.player.item.clone() {
                        if let Some(&Shop {
                            shop_type:
                                ShopType::Sell {
                                    require_item,
                                    give_item,
                                    needs_coin,
                                },
                            ..
                        }) = shop
                        {
                            if require_item == item.item_type
                                && (!needs_coin || self.player.money > 0)
                            {
                                if needs_coin {
                                    self.player.money -= 1;
                                }
                                let item_id = item.id;
                                self.player.item = None;
                                if let Some(item_type) = give_item {
                                    let mut effect = self.assets.change.effect();
                                    effect.set_volume(self.ui_state.volume);
                                    effect.play();
                                    self.player.item = Some(Item {
                                        id: item_id,
                                        position: self.player.position,
                                        item_type,
                                        value: 0,
                                    })
                                } else {
                                    let mut effect = self.assets.money.effect();
                                    effect.set_volume(self.ui_state.volume);
                                    effect.play();
                                    self.player.money += item.value;
                                }
                            }
                        }
                    } else {
                        let closest_item =
                            self.model.items.values().min_by_key(|item| {
                                r32((item.position - self.player.position).len())
                            });
                        if let Some(item) = closest_item {
                            if (item.position - self.player.position).len() < Player::RANGE {
                                self.player.item = Some(item.clone());
                                self.to_send
                                    .push(ClientMessage::Event(Event::ItemRemoved(item.id)));
                            }
                        }
                    }
                }
                geng::Key::Q => {
                    if let Some(mut item) = self.player.item.take() {
                        item.position = self.player.position;
                        self.to_send
                            .push(ClientMessage::Event(Event::ItemAdded(item)));
                    }
                }
                _ => {}
            },
            geng::Event::MouseDown {
                button, position, ..
            } => {
                if self.music.is_none() && self.connection.is_local() {
                    self.music = Some({
                        let mut music = self.assets.music.play();
                        music.set_volume(0.2);
                        music
                    });
                }
                let position = self
                    .camera
                    .screen_to_world(self.framebuffer_size, position.map(|x| x as f32));
                let position = position.map(|x| x.floor() as i32);
                match button {
                    geng::MouseButton::Right => {
                        if let Some(item) = &self.player.item {
                            if !self.model.tiles.contains_key(&position) {
                                if let Some(tile) = item.item_type.placed() {
                                    if ((self.player.position + self.player.size / 2.0)
                                        - position.map(|x| x as f32 + 0.5))
                                    .len()
                                        < Player::RANGE
                                    {
                                        self.to_send.push(ClientMessage::Event(Event::TilePlaced(
                                            position, tile,
                                        )));
                                        self.player.item = None;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }
}
