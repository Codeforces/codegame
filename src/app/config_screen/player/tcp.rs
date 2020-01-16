use super::*;

type TcpPlayerFuture<G: Game> = dyn Future<Output = Result<TcpPlayer<G>, std::io::Error>>;

pub struct TcpPlayerConfig<G: Game> {
    geng: Rc<Geng>,
    theme: Rc<ui::Theme>,
    options: TcpPlayerOptions,
    port_buttons: [ui::TextButton; 2],
    player: Option<Pin<Box<futures::future::MaybeDone<Pin<Box<TcpPlayerFuture<G>>>>>>>,
}

impl<G: Game> TcpPlayerConfig<G> {
    pub fn new(geng: &Rc<Geng>, theme: &Rc<ui::Theme>) -> Self {
        Self {
            geng: geng.clone(),
            theme: theme.clone(),
            options: TcpPlayerOptions {
                host: None,
                port: 31001,
                accept_timeout: None,
                timeout: None,
                token: None,
            },
            port_buttons: [
                ui::TextButton::new(geng, theme, "<".to_owned(), 24.0),
                ui::TextButton::new(geng, theme, ">".to_owned(), 24.0),
            ],
            player: None,
        }
    }
    pub fn constructor(
        geng: &Rc<Geng>,
        theme: &Rc<ui::Theme>,
    ) -> Box<dyn Fn() -> Box<dyn PlayerConfig<G>>> {
        let geng = geng.clone();
        let theme = theme.clone();
        Box::new(move || Box::new(Self::new(&geng, &theme)))
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
        let state = Rc::new(RefCell::new((&mut self.options.port, &mut self.player)));
        let mut port_buttons = self.port_buttons.iter_mut();
        let port_config = ui::row![
            ui::text(
                format!("{}: {}", translate("Port"), *state.borrow().0),
                &self.theme.font,
                16.0,
                Color::GRAY
            )
            .padding_right(8.0)
            .align(vec2(0.5, 0.5)),
            port_buttons
                .next()
                .unwrap()
                .ui(Box::new({
                    let state = state.clone();
                    move || {
                        *state.borrow_mut().0 -= 1;
                        *state.borrow_mut().1 = None;
                    }
                }))
                .align(vec2(0.5, 0.5)),
            port_buttons
                .next()
                .unwrap()
                .ui(Box::new({
                    let state = state.clone();
                    move || {
                        *state.borrow_mut().0 += 1;
                        *state.borrow_mut().1 = None;
                    }
                }))
                .align(vec2(0.5, 0.5)),
        ];
        let text =
            ui::text(status_text, &self.theme.font, 16.0, status_color).align(vec2(0.5, 1.0));
        Box::new(ui::column![port_config.align(vec2(0.5, 0.5)), text])
    }
    fn ready(&mut self) -> bool {
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
