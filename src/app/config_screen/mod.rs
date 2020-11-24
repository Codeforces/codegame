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
    fn debug_state(&self, game: &G, player_index: usize) -> G::DebugState {
        self.0.borrow().debug_state(game, player_index)
    }
}

pub trait DeepConfig<T>: ui::Config<T> {
    fn transition(&mut self) -> Option<Box<dyn geng::State>>;
}

struct Data<G: Game, R: Renderer<G>> {
    theme: Rc<ui::Theme>,
    preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
    game_options_config: Box<dyn DeepConfig<G::OptionsPreset>>,
    #[cfg(not(target_arch = "wasm32"))]
    game_state_path: Option<std::path::PathBuf>,
    #[cfg(not(target_arch = "wasm32"))]
    game_state_path_button: ui::Button,
    #[cfg(not(target_arch = "wasm32"))]
    save_button: ui::Button,
    #[cfg(not(target_arch = "wasm32"))]
    replay_button: ui::Button,
    start_button: ui::Button,
    repeat_button: ui::Button,
    player_count_range: RangeInclusive<usize>,
    player_configs: Vec<PlayerConfigWidget<G>>,
    player_config_options: Rc<Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>>,
    add_player_button: ui::Button,
    remove_player_button: ui::Button,
    renderer: RendererWrapper<R>,
}

impl<G: Game, R: Renderer<G>> Data<G, R> {
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        if self.add_player_button.clicked() {
            if self.player_configs.len() < *self.player_count_range.end() {
                self.player_configs.push(PlayerConfigWidget::new(
                    &self.theme,
                    &self.player_config_options,
                    0,
                ));
            }
        }
        if self.remove_player_button.clicked() {
            if self.player_configs.len() > *self.player_count_range.start() {
                self.player_configs.pop();
            }
        }

        use ui::*;
        #[cfg(not(target_arch = "wasm32"))]
        let options = self.full_options();
        let mut ready = true;
        for config in &mut self.player_configs {
            // Doing for to assert calling .ready()
            if !config.ready() {
                ready = false;
            }
        }
        let play_section = if ready {
            let play = ui::Button::text(&mut self.start_button, translate("START"), &self.theme);
            if cfg!(target_arch = "wasm32") {
                Box::new(play) as Box<dyn Widget>
            } else {
                Box::new(ui::row![
                    play.center(),
                    ui::Text::new(translate("or"), &self.theme.font, 32.0, Color::GRAY)
                        .center()
                        .padding_left(32.0)
                        .padding_right(32.0),
                    ui::Button::text(
                        &mut self.repeat_button,
                        translate("repeat a game"),
                        &self.theme
                    )
                    .center(),
                ]) as Box<dyn Widget>
            }
        } else {
            Box::new(ui::Text::new(
                translate("Waiting for players"),
                &self.theme.font,
                32.0,
                Color::GRAY,
            )) as Box<dyn Widget>
        };
        let theme = &self.theme;
        let players_section = ui::row(
            self.player_configs
                .iter_mut()
                .enumerate()
                .map(move |(index, config)| {
                    Box::new({
                        ui::column![
                            ui::Text::new(
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
                        ui::Button::text(
                            &mut self.add_player_button,
                            translate("add player"),
                            &self.theme
                        ),
                        ui::Button::text(
                            &mut self.remove_player_button,
                            translate("remove player"),
                            &self.theme
                        ),
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
                ui::Text::new(
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
                ui::Button::text(
                    &mut self.game_state_path_button,
                    translate("Load game state"),
                    &self.theme
                ),
            ]
            .center(),
            config_section,
        ];
        let config_section = ui::column![
            ui::Text::new(
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
                ui::Text::new(
                    translate("Create a game or"),
                    &self.theme.font,
                    32.0,
                    Color::GRAY
                )
                .padding_right(32.0),
                ui::Button::text(
                    &mut self.replay_button,
                    translate("watch a replay"),
                    &self.theme
                ),
            ]
            .center()
            .padding_bottom(32.0),
            result,
            {
                if self.save_button.clicked() {
                    save_file(translate("save config"), "config.json", move |writer| {
                        options.save(writer)
                    })
                    .expect("Failed to save config");
                }
                ui::Button::text(&mut self.save_button, translate("save config"), &self.theme)
                    .padding_top(32.0)
                    .center()
            },
        ];
        result.center()
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if self.start_button.clicked() {
            let players: Vec<Box<dyn Player<G>>> = self
                .player_configs
                .iter_mut()
                .map(|config| config.create())
                .collect();
            return Some(geng::Transition::Push(Box::new(GameScreen::new(
                self.theme.geng(),
                GameProcessor::new(None, self.game_init_config().into(), players),
                self.renderer.clone(),
                self.preferences.clone(),
            ))));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.replay_button.clicked() {
                if let Some(path) = select_file(translate("Select file to replay")) {
                    return Some(geng::Transition::Push(Box::new(GameScreen::replay(
                        self.theme.geng(),
                        futures::executor::block_on(History::load(path.to_str().unwrap())),
                        self.renderer.clone(),
                        self.preferences.clone(),
                    ))));
                }
            }
            if self.repeat_button.clicked() {
                if let Some(path) = select_file(translate("Select file to repeat")) {
                    let players: Vec<Box<dyn Player<G>>> = self
                        .player_configs
                        .iter_mut()
                        .map(|config| config.create())
                        .collect();
                    return Some(geng::Transition::Push(Box::new(GameScreen::new(
                        self.theme.geng(),
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
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(path) = &self.game_state_path {
            return GameInitConfig::LoadFrom(path.clone());
        }
        GameInitConfig::Create(self.game_options_config.get())
    }
    #[cfg(not(target_arch = "wasm32"))]
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
                theme,
                &player_config_options,
                default,
            ));
        }
        Self {
            data: Data {
                theme: theme.clone(),
                #[cfg(not(target_arch = "wasm32"))]
                replay_button: ui::Button::new(),
                #[cfg(not(target_arch = "wasm32"))]
                save_button: ui::Button::new(),
                start_button: ui::Button::new(),
                repeat_button: ui::Button::new(),
                game_options_config,
                #[cfg(not(target_arch = "wasm32"))]
                game_state_path: None,
                #[cfg(not(target_arch = "wasm32"))]
                game_state_path_button: ui::Button::new(),
                player_count_range,
                player_configs,
                player_config_options: player_config_options.clone(),
                add_player_button: ui::Button::new(),
                remove_player_button: ui::Button::new(),
                renderer,
                preferences,
            },
            ui_controller: ui::Controller::new(),
        }
    }
}

impl<G: Game, R: Renderer<G>> geng::State for ConfigScreen<G, R> {
    fn update(&mut self, delta_time: f64) {
        self.ui_controller.update(&mut self.data.ui(), delta_time);
    }
    fn handle_event(&mut self, event: geng::Event) {
        self.ui_controller.handle_event(&mut self.data.ui(), event);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.ui_controller.draw(&mut self.data.ui(), framebuffer);
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if let Some(transition) = self.data.game_options_config.transition() {
            return Some(geng::Transition::Push(transition));
        }
        self.data.transition()
    }
}
