use super::*;

pub struct Lobby {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    transition: Option<geng::Transition>,
}

impl Lobby {
    pub fn new(geng: &Rc<Geng>, assets: Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets,
            transition: None,
        }
    }
}

impl geng::State for Lobby {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::rgb(0.0, 0.0, 1.0)), None);
    }
    fn update(&mut self, delta_time: f64) {}
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key, .. } => match key {
                geng::Key::Num1 => {
                    let mut model = Model::new();
                    let (welcome, _) = model.welcome();
                    self.transition = Some(geng::Transition::Push(Box::new(GameState::new(
                        &self.geng,
                        &self.assets,
                        welcome,
                        Connection::Local {
                            next_tick: 0.0,
                            model,
                        },
                    ))));
                }
                geng::Key::Num2 => {
                    self.transition = Some(geng::Transition::Push(Box::new(ConnectingState::new(
                        &self.geng,
                        &self.assets,
                    ))));
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

pub struct ConnectingState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    connection: Option<Pin<Box<dyn Future<Output = (WelcomeMessage, Connection)>>>>,
    transition: Option<geng::Transition>,
}

impl ConnectingState {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        let addr = format!("{}://{}", option_env!("WSS").unwrap_or("ws"), SERVER_ADDR);
        let connection = Box::pin(
            geng::net::client::connect(&addr)
                .then(|connection| async move {
                    let (message, connection) = connection.into_future().await;
                    let welcome = match message {
                        Some(ServerMessage::Welcome(message)) => message,
                        _ => unreachable!(),
                    };
                    (welcome, connection)
                })
                .map(|(welcome, connection)| (welcome, Connection::Remote(connection))),
        );
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            connection: Some(connection),
            transition: None,
        }
    }
}

impl geng::State for ConnectingState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::GREEN), None);
    }
    fn update(&mut self, delta_time: f64) {}
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
        if let Some(connection) = &mut self.connection {
            if let std::task::Poll::Ready((welcome, connection)) =
                connection
                    .as_mut()
                    .poll(&mut std::task::Context::from_waker(
                        futures::task::noop_waker_ref(),
                    ))
            {
                return Some(geng::Transition::Switch(Box::new(GameState::new(
                    &self.geng,
                    &self.assets,
                    welcome,
                    connection,
                ))));
            }
        }
        self.transition.take()
    }
}