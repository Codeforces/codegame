use super::*;

type TcpPlayerFuture<G> = dyn Future<Output = Result<TcpPlayer<G>, std::io::Error>>;

pub struct TcpPlayerConfig<G: Game> {
    theme: Rc<ui::Theme>,
    options: TcpPlayerOptions,
    port_buttons: [ui::Button; 2],
    player: Option<Pin<Box<futures::future::MaybeDone<Pin<Box<TcpPlayerFuture<G>>>>>>>,
}

impl<G: Game> TcpPlayerConfig<G> {
    pub fn new(theme: &Rc<ui::Theme>) -> Self {
        Self {
            theme: theme.clone(),
            options: TcpPlayerOptions {
                host: None,
                port: 31001,
                accept_timeout: None,
                timeout: None,
                token: None,
            },
            port_buttons: [ui::Button::new(), ui::Button::new()],
            player: None,
        }
    }
    pub fn constructor(theme: &Rc<ui::Theme>) -> Box<dyn Fn() -> Box<dyn PlayerConfig<G>>> {
        let theme = theme.clone();
        Box::new(move || Box::new(Self::new(&theme)))
    }
}

impl<G: Game> PlayerConfig<G> for TcpPlayerConfig<G> {
    fn name(&self) -> &str {
        "TCP"
    }
    fn ui<'a>(&'a mut self) -> Box<dyn ui::Widget + 'a> {
        use ui::*;
        let (status_text, status_color) = if let Some(player) = &mut self.player {
            if let Some(result) = player.as_mut().output_mut() {
                match result {
                    Ok(_) => (translate("Player connected"), Color::GREEN),
                    Err(_) => (translate("Failed to listen specified port"), Color::RED),
                }
            } else {
                (translate("Waiting for connection"), Color::YELLOW)
            }
        } else {
            ("", Color::WHITE)
        };
        let mut port_buttons = self.port_buttons.iter_mut();
        let port_config = ui::row![
            ui::Text::new(
                format!("{}: {}", translate("Port"), self.options.port),
                &self.theme.font,
                16.0,
                Color::GRAY
            )
            .padding_right(8.0)
            .center(),
            ui::Button::text(port_buttons.next().unwrap(), "<", &self.theme).center(),
            ui::Button::text(port_buttons.next().unwrap(), ">", &self.theme).center(),
        ];
        let text =
            ui::Text::new(status_text, &self.theme.font, 16.0, status_color).align(vec2(0.5, 1.0));
        Box::new(ui::column![port_config.center(), text])
    }
    fn ready(&mut self) -> bool {
        if self.port_buttons[0].clicked() {
            self.options.port -= 1;
            self.player = None;
        }
        if self.port_buttons[1].clicked() {
            self.options.port += 1;
            self.player = None;
        }
        if self.player.is_none() {
            self.player = Some(Box::pin(futures::future::maybe_done(
                TcpPlayer::new(self.options.clone()).boxed_local(),
            )));
        }
        let _ = self
            .player
            .as_mut()
            .unwrap()
            .as_mut()
            .poll(&mut std::task::Context::from_waker(
                futures::task::noop_waker_ref(),
            ));
        if let futures::future::MaybeDone::Done(_) = **self.player.as_ref().unwrap() {
            self.player
                .as_mut()
                .unwrap()
                .as_mut()
                .output_mut()
                .unwrap()
                .is_ok()
        } else {
            false
        }
    }
    fn get(&mut self) -> Box<dyn Player<G>> {
        assert!(<Self as PlayerConfig<G>>::ready(self));
        let player = self
            .player
            .take()
            .unwrap()
            .as_mut()
            .take_output()
            .unwrap()
            .unwrap();
        Box::new(player)
    }
    fn to_options(&self) -> G::PlayerOptions {
        self.options.clone().into()
    }
}
