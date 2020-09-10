use super::*;

mod player;

pub use player::*;

struct RendererWrapper<R>(Rc<RefCell<R>>);

impl<R> Clone for RendererWrapper<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<G: Game, R: Renderer<G>> Renderer<G> for RendererWrapper<R> {
    type ExtraData = R::ExtraData;
    type Preferences = R::Preferences;
    fn default_tps(&self) -> f64 {
        self.0.borrow().default_tps()
    }
    fn update(&mut self, delta_time: f64) {
        self.0.borrow_mut().update(delta_time);
    }
    fn draw(&mut self, state: RenderState<G, R::ExtraData>, framebuffer: &mut ugli::Framebuffer) {
        self.0.borrow_mut().draw(state, framebuffer);
    }
    fn process_event(&mut self, event: &G::Event) {
        self.0.borrow_mut().process_event(event);
    }
    fn handle_event(&mut self, event: &geng::Event) {
        self.0.borrow_mut().handle_event(event);
    }
}

pub trait DeepConfig<T>: ui::Config<T> {
    fn transition(&mut self) -> Option<Box<dyn geng::State>>;
}

struct Data<G: Game, R: Renderer<G>> {
    geng: Rc<Geng>,
    preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
    theme: Rc<ui::Theme>,
    game_options_config: Box<dyn DeepConfig<G::OptionsPreset>>,
    game_state_path: Option<std::path::PathBuf>,
    game_state_path_button: ui::TextButton,
    save_button: ui::TextButton,
    replay_button: ui::TextButton,
    start_button: ui::TextButton,
    repeat_button: ui::TextButton,
    player_count_range: RangeInclusive<usize>,
    player_configs: Vec<PlayerConfigWidget<G>>,
    player_config_options: Rc<Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>>,
    add_player_button: ui::TextButton,
    add_player: bool,
    remove_player_button: ui::TextButton,
    remove_player: bool,
    renderer: RendererWrapper<R>,
    replay_requested: bool,
    repeat_requested: bool,
    ready: bool,
}

impl<G: Game, R: Renderer<G>> Data<G, R> {
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        if self.add_player {
            self.add_player = false;
            if self.player_configs.len() < *self.player_count_range.end() {
                self.player_configs.push(PlayerConfigWidget::new(
                    &self.geng,
                    &self.theme,
                    &self.player_config_options,
                    0,
                ));
            }
        }
        if self.remove_player {
            self.remove_player = false;
            if self.player_configs.len() > *self.player_count_range.start() {
                self.player_configs.pop();
            }
        }

        use ui::*;
        let options = self.full_options();
        let mut ready = true;
        for config in &mut self.player_configs {
            // Doing for to assert calling .ready()
            if !config.ready() {
                ready = false;
            }
        }
        let play_section = if ready {
            let play = self.start_button.ui(Box::new({
                let ready = &mut self.ready;
                move || *ready = true
            }));
            if cfg!(target_arch = "wasm32") {
                Box::new(play) as Box<dyn Widget>
            } else {
                Box::new(ui::row![
                    play.center(),
                    text(translate("or"), &self.theme.font, 32.0, Color::GRAY)
                        .center()
                        .padding_left(32.0)
                        .padding_right(32.0),
                    self.repeat_button
                        .ui(Box::new({
                            let repeat_requested = &mut self.repeat_requested;
                            move || *repeat_requested = true
                        }))
                        .center(),
                ]) as Box<dyn Widget>
            }
        } else {
            Box::new(text(
                translate("Waiting for players"),
                &self.theme.font,
                32.0,
                Color::GRAY,
            )) as Box<dyn Widget>
        };
        let theme = &self.theme;
        let add_player = &mut self.add_player;
        let remove_player = &mut self.remove_player;
        let players_section = ui::row(
            self.player_configs
                .iter_mut()
                .enumerate()
                .map(move |(index, config)| {
                    Box::new({
                        ui::column![
                            ui::text(
                                format!("{} {}", translate("Player"), index + 1),
                                &theme.font,
                                32.0,
                                Color::GRAY,
                            )
                            .center(),
                            config.ui().center()
                        ]
                        .fixed_size(vec2(200.0, 100.0))
                        .center()
                    }) as _
                })
                .chain(std::iter::once(Box::new(
                    ui::column![
                        self.add_player_button
                            .ui(Box::new(move || *add_player = true)),
                        self.remove_player_button
                            .ui(Box::new(move || *remove_player = true)),
                    ]
                    .center(),
                ) as _))
                .collect(),
        )
        .center();
        let config_section = self.game_options_config.ui().center();
        #[cfg(not(target_arch = "wasm32"))]
        let config_section = ui::column![
            ui::row![
                text(
                    self.game_state_path
                        .as_ref()
                        .map_or("Create new game", |path| path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap())
                        .to_owned(),
                    &self.theme.font,
                    32.0,
                    Color::GRAY
                )
                .padding_right(32.0),
                self.game_state_path_button.ui(Box::new({
                    let path = &mut self.game_state_path;
                    move || {
                        *path = select_file("Load game state");
                    }
                })),
            ]
            .center(),
            config_section,
        ];
        let config_section = ui::column![
            text(
                translate("Game options"),
                &self.theme.font,
                32.0,
                Color::GRAY,
            )
            .padding_top(32.0)
            .center(),
            config_section,
        ];
        let play_section = play_section.padding_top(32.0).center();
        let result = ui::column![players_section, config_section, play_section];
        #[cfg(not(target_arch = "wasm32"))]
        let result = ui::column![
            row![
                text(
                    translate("Create a game or"),
                    &self.theme.font,
                    32.0,
                    Color::GRAY
                )
                .padding_right(32.0),
                self.replay_button.ui(Box::new({
                    let replay_requested = &mut self.replay_requested;
                    move || *replay_requested = true
                })),
            ]
            .center()
            .padding_bottom(32.0),
            result,
            self.save_button
                .ui(Box::new(move || {
                    save_file(translate("save config"), "config.json", move |writer| {
                        options.save(writer)
                    })
                    .expect("Failed to save config");
                }))
                .padding_top(32.0)
                .center(),
        ];
        result.center()
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if self.ready {
            self.ready = false;
            let players: Vec<Box<dyn Player<G>>> = self
                .player_configs
                .iter_mut()
                .map(|config| config.create())
                .collect();
            return Some(geng::Transition::Push(Box::new(GameScreen::new(
                &self.geng,
                GameProcessor::new(None, self.game_init_config().into(), players),
                self.renderer.clone(),
                self.preferences.clone(),
            ))));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.replay_requested {
                self.replay_requested = false;
                if let Some(path) = select_file(translate("Select file to replay")) {
                    return Some(geng::Transition::Push(Box::new(GameScreen::replay(
                        &self.geng,
                        futures::executor::block_on(History::load(path.to_str().unwrap())),
                        self.renderer.clone(),
                        self.preferences.clone(),
                    ))));
                }
            }
            if self.repeat_requested {
                self.repeat_requested = false;
                if let Some(path) = select_file(translate("Select file to repeat")) {
                    let players: Vec<Box<dyn Player<G>>> = self
                        .player_configs
                        .iter_mut()
                        .map(|config| config.create())
                        .collect();
                    return Some(geng::Transition::Push(Box::new(GameScreen::new(
                        &self.geng,
                        GameProcessor::<G>::repeat(
                            std::fs::File::open(path).expect("Failed to open game log file"),
                            players,
                        ),
                        self.renderer.clone(),
                        self.preferences.clone(),
                    ))));
                }
            }
        }
        None
    }
    fn game_init_config(&self) -> GameInitConfig<G> {
        match &self.game_state_path {
            Some(path) => GameInitConfig::LoadFrom(path.clone()),
            Non => GameInitConfig::Create(self.game_options_config.get()),
        }
    }
    fn full_options(&self) -> FullOptions<G> {
        FullOptions {
            seed: None,
            game: self.game_init_config(),
            players: self
                .player_configs
                .iter()
                .map(|config| config.to_options())
                .collect(),
        }
    }
}

pub struct ConfigScreen<G: Game, R: Renderer<G>> {
    data: Data<G, R>,
    ui_controller: ui::Controller,
}

impl<G: Game, R: Renderer<G>> ConfigScreen<G, R> {
    pub fn new(
        geng: &Rc<Geng>,
        theme: &Rc<ui::Theme>,
        game_options_config: Box<dyn DeepConfig<G::OptionsPreset>>,
        player_config_options: Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>,
        player_config_defaults: Vec<usize>,
        player_count_range: RangeInclusive<usize>,
        renderer: R,
        preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
    ) -> Self {
        add_translations(include_str!("translations.txt"));
        let renderer = RendererWrapper(Rc::new(RefCell::new(renderer)));
        let player_config_options = Rc::new(player_config_options);
        let mut player_configs = Vec::with_capacity(player_config_defaults.len());
        for default in player_config_defaults {
            player_configs.push(PlayerConfigWidget::new(
                geng,
                theme,
                &player_config_options,
                default,
            ));
        }
        Self {
            data: Data {
                geng: geng.clone(),
                theme: theme.clone(),
                replay_button: ui::TextButton::new(
                    geng,
                    theme,
                    translate("watch a replay").to_owned(),
                    32.0,
                ),
                save_button: ui::TextButton::new(
                    geng,
                    theme,
                    translate("save config").to_owned(),
                    24.0,
                ),
                start_button: ui::TextButton::new(geng, theme, translate("START").to_owned(), 32.0),
                repeat_button: ui::TextButton::new(
                    geng,
                    theme,
                    translate("repeat a game").to_owned(),
                    32.0,
                ),
                game_options_config,
                game_state_path: None,
                game_state_path_button: ui::TextButton::new(
                    geng,
                    theme,
                    translate("load game state").to_owned(),
                    32.0,
                ),
                player_count_range,
                player_configs,
                player_config_options: player_config_options.clone(),
                add_player_button: ui::TextButton::new(
                    geng,
                    theme,
                    translate("add").to_owned(),
                    32.0,
                ),
                add_player: false,
                remove_player_button: ui::TextButton::new(
                    geng,
                    theme,
                    translate("remove").to_owned(),
                    32.0,
                ),
                remove_player: false,
                renderer,
                replay_requested: false,
                repeat_requested: false,
                ready: false,
                preferences,
            },
            ui_controller: ui::Controller::new(),
        }
    }
}

impl<G: Game, R: Renderer<G>> geng::State for ConfigScreen<G, R> {
    fn update(&mut self, delta_time: f64) {
        self.ui_controller.update(self.data.ui(), delta_time);
    }
    fn handle_event(&mut self, event: geng::Event) {
        self.ui_controller.handle_event(self.data.ui(), event);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.ui_controller.draw(self.data.ui(), framebuffer);
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if let Some(transition) = self.data.game_options_config.transition() {
            return Some(geng::Transition::Push(transition));
        }
        self.data.transition()
    }
}
