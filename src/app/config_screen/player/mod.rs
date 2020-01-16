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
    geng: Rc<Geng>,
    theme: Rc<ui::Theme>,
    options: Rc<Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>>,
    current_config: Box<dyn PlayerConfig<G>>,
    current_config_index: usize,
    config_switch: Option<usize>,
    label: String,
    button: ui::TextButton,
}

impl<G: Game> PlayerConfigWidget<G> {
    pub fn new(
        geng: &Rc<Geng>,
        theme: &Rc<ui::Theme>,
        options: &Rc<Vec<Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>>>,
        default_option: usize,
        label: String,
    ) -> Self {
        let current_config = options[default_option]();
        let button_text = current_config.name().to_owned();
        Self {
            geng: geng.clone(),
            theme: theme.clone(),
            options: options.clone(),
            current_config,
            current_config_index: default_option,
            config_switch: None,
            label,
            button: ui::TextButton::new(geng, theme, button_text, 32.0),
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        if let Some(index) = self.config_switch.take() {
            self.current_config = self.options[index]();
            self.button.text = self.current_config.name().to_owned();
        }
        ui::column![
            text(&self.label, &self.theme.font, 32.0, Color::GRAY).align(vec2(0.5, 0.5)),
            self.button
                .ui(Box::new({
                    let new_config = (self.current_config_index + 1) % self.options.len();
                    let config_index = &mut self.current_config_index;
                    let config_switch = &mut self.config_switch;
                    move || {
                        *config_index = new_config;
                        *config_switch = Some(new_config);
                    }
                }))
                .align(vec2(0.5, 0.5))
                .uniform_padding(8.0),
            self.current_config.ui(),
        ]
        .fixed_size(vec2(200.0, 100.0))
        .align(vec2(0.5, 0.5))
    }
    pub fn ready(&mut self) -> bool {
        self.current_config.ready()
    }
    pub fn create(&mut self) -> Box<dyn Player<G>> {
        self.current_config.get()
    }
    pub fn to_options(&self) -> G::PlayerOptions {
        self.current_config.to_options()
    }
}
