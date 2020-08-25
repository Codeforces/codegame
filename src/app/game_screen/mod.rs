use super::*;

mod ui;

pub struct GameScreen<G: Game, R: Renderer<G>> {
    geng: Rc<Geng>,
    processor: Option<BackgroundGameProcessor<G>>,
    renderer: R,
    history: History<G, R::ExtraData>,
    current_tick: f64,
    paused: Rc<Cell<bool>>,
    view_speed_modifier: Rc<Cell<f64>>,
    volume: Rc<Cell<f64>>,
    ui: ui::UI,
    ui_controller: geng::ui::Controller,
    need_close: bool,
    preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences<T> {
    pub view_speed_modifier: f64,
    pub volume: f64,
    pub renderer: T,
}

impl<T: Default> Default for AppPreferences<T> {
    fn default() -> Self {
        Self {
            volume: 0.5,
            view_speed_modifier: 0.0,
            renderer: default(),
        }
    }
}

impl<G: Game, R: Renderer<G>> GameScreen<G, R> {
    fn new_impl(
        geng: &Rc<Geng>,
        history: History<G, R::ExtraData>,
        processor: Option<GameProcessor<G>>,
        renderer: R,
        preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
    ) -> Self {
        add_translations(include_str!("translations.txt"));
        let paused = Rc::new(Cell::new(false));
        let view_speed_modifier = Rc::new(Cell::new(preferences.borrow().view_speed_modifier));
        let volume = Rc::new(Cell::new(preferences.borrow().volume));
        let processor = processor.map(|processor| {
            BackgroundGameProcessor::new(
                processor,
                history.tick_handler(),
                Some(history.client_data_handler()),
            )
        });
        Self {
            geng: geng.clone(),
            processor,
            renderer,
            history,
            current_tick: 0.0,
            paused: paused.clone(),
            view_speed_modifier: view_speed_modifier.clone(),
            volume: volume.clone(),
            ui: ui::UI::new(geng, &paused, &view_speed_modifier, &volume),
            ui_controller: geng::ui::Controller::new(),
            need_close: false,
            preferences,
        }
    }
    pub fn new(
        geng: &Rc<Geng>,
        processor: GameProcessor<G>,
        renderer: R,
        preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
    ) -> Self {
        let history = History::new(processor.game().clone());
        Self::new_impl(geng, history, Some(processor), renderer, preferences)
    }
    pub fn replay(
        geng: &Rc<Geng>,
        history: History<G, R::ExtraData>,
        renderer: R,
        preferences: Rc<RefCell<AutoSave<AppPreferences<R::Preferences>>>>,
    ) -> Self {
        Self::new_impl(geng, history, None, renderer, preferences)
    }
}

impl<G: Game, R: Renderer<G>> geng::State for GameScreen<G, R>
where
    Self: 'static,
{
    fn update(&mut self, delta_time: f64) {
        if self.view_speed_modifier.get() != self.preferences.borrow().view_speed_modifier {
            self.preferences.borrow_mut().view_speed_modifier = self.view_speed_modifier.get();
        }
        if self.volume.get() != self.preferences.borrow().volume {
            self.preferences.borrow_mut().volume = self.volume.get();
        }
        let history_len = self.history.len();
        self.current_tick = self.current_tick.min(history_len as f64);
        let mut process_events = false;
        if let Some(time) = self.ui.timeline_change() {
            self.current_tick = time;
        } else {
            if !self.paused.get() {
                self.current_tick += delta_time
                    * ui::view_speed(self.view_speed_modifier.get(), self.renderer.default_tps());
                process_events = true;
            }
        }

        if let Some(processor) = &mut self.processor {
            processor.proceed({
                let tick_needed = self.current_tick.ceil() as usize;
                if tick_needed >= history_len {
                    tick_needed - history_len + 1
                } else {
                    0
                }
            });
        }

        let max_time = (history_len.max(2) - 1) as f64;
        self.ui.set_time(
            self.current_tick.min(max_time),
            max_time,
            self.renderer.default_tps(),
        );
        self.ui_controller
            .update(self.ui.ui(self.renderer.default_tps()), delta_time);

        for event in self.history.go_to(self.current_tick, process_events) {
            self.renderer.process_event(&event);
        }

        self.renderer.update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let state = self.history.current_state();
        self.renderer.draw(state, framebuffer);
        self.ui_controller
            .draw(self.ui.ui(self.renderer.default_tps()), framebuffer);
    }
    fn handle_event(&mut self, event: geng::Event) {
        if !self
            .ui_controller
            .handle_event(self.ui.ui(self.renderer.default_tps()), event.clone())
        {
            if !match event {
                geng::Event::KeyDown { key } => match key {
                    geng::Key::P | geng::Key::Space => {
                        self.paused.set(!self.paused.get());
                        true
                    }
                    geng::Key::F => {
                        self.geng.window().toggle_fullscreen();
                        true
                    }
                    geng::Key::Left if self.paused.get() => {
                        self.current_tick = partial_max(self.current_tick - 1.0, 0.0);
                        true
                    }
                    geng::Key::Right if self.paused.get() => {
                        self.current_tick += 1.0;
                        true
                    }
                    geng::Key::Escape => {
                        self.need_close = true;
                        true
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    geng::Key::S if self.geng.window().is_key_pressed(geng::Key::LCtrl) => {
                        save_file(translate("Save game log"), "game.log", |mut writer| {
                            self.history.save(&mut writer)
                        })
                        .expect("Failed to save game log");
                        true
                    }
                    _ => false,
                },
                _ => false,
            } {
                self.renderer.handle_event(&event);
            }
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if self.need_close {
            Some(geng::Transition::Pop)
        } else {
            None
        }
    }
}
