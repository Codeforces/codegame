use super::*;

mod empty;
mod tcp;

pub use empty::*;
pub use tcp::*;

pub trait PlayerConfig<G: Game> {
    fn name(&self) -> &str;
    fn ui<'a>(&'a mut self) -> Box<dyn ui::Widget + 'a>;
    fn ready(&mut self) -> bool;
    fn get(&mut self) -> Box<dyn Player<G>>;
    fn to_options(&self) -> G::PlayerOptions;
}

pub(crate) struct PlayerConfigWidget<G: Game> {
    theme: Rc<ui::Theme>,
    options: Rc<Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>>,
    current_config: Box<dyn PlayerConfig<G>>,
    current_config_index: usize,
    button: ui::Button,
}

impl<G: Game> PlayerConfigWidget<G> {
    pub fn new(
        theme: &Rc<ui::Theme>,
        options: &Rc<Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>>,
        default_option: usize,
    ) -> Self {
        let current_config = options[default_option]();
        Self {
            theme: theme.clone(),
            options: options.clone(),
            current_config,
            current_config_index: default_option,
            button: ui::Button::new(),
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        if self.button.clicked() {
            self.current_config_index = (self.current_config_index + 1) % self.options.len();
            self.current_config = self.options[self.current_config_index]();
        }
        let current_config_name = self.current_config.name().to_owned();
        ui::column![
            ui::Button::text(&mut self.button, current_config_name, &self.theme)
                .center()
                .uniform_padding(8.0),
            self.current_config.ui(),
        ]
    }
    pub fn ready(&mut self) -> bool {
        self.current_config.ready()
    }
    pub fn create(&mut self) -> Box<dyn Player<G>> {
        self.current_config.get()
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_options(&self) -> G::PlayerOptions {
        self.current_config.to_options()
    }
}
