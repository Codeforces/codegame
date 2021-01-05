use super::*;

struct TimeLabel {
    font: Rc<geng::Font>,
    tick_text: String,
    time_text: String,
}

impl TimeLabel {
    fn new(theme: &Rc<ui::Theme>) -> Self {
        Self {
            font: theme.font.clone(),
            tick_text: String::new(),
            time_text: String::new(),
        }
    }
    fn ui<'a>(&'a mut self, tick_time: f64, ticks_per_second: f64) -> impl ui::Widget + 'a {
        use ui::*;
        self.tick_text = format!("{}", tick_time as usize);
        let time = tick_time / ticks_per_second;
        let time = time as usize;
        self.time_text = format!("{}:{:02}", time / 60, time % 60);
        ui::column![
            ui::Text::new(
                &self.tick_text,
                &self.font,
                UI_SIZE as f32 / 2.0,
                Color::WHITE
            )
            .maintain_aspect(vec2(0.5, 0.5)),
            ui::Text::new(
                &self.time_text,
                &self.font,
                UI_SIZE as f32 / 2.0,
                Color::WHITE
            )
            .maintain_aspect(vec2(0.5, 0.5)),
        ]
        .fixed_size(vec2(UI_SIZE, UI_SIZE))
    }
}

#[derive(Deref)]
pub struct Timeline {
    #[deref]
    slider: ui::Slider,
    label: TimeLabel,
    time: f64,
    max_time: f64,
    ticks_per_second: f64,
}

impl Timeline {
    pub fn new(theme: &Rc<ui::Theme>) -> Self {
        Self {
            slider: ui::Slider::new(theme),
            label: TimeLabel::new(theme),
            time: 0.0,
            max_time: 1.0,
            ticks_per_second: 1.0,
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use geng::ui::*;
        let time = &mut self.time;
        row![
            self.label
                .ui(*time, self.ticks_per_second)
                .uniform_padding(UI_PADDING),
            self.slider
                .ui(
                    *time,
                    0.0..=self.max_time,
                    Box::new(move |value| *time = value),
                )
                .constraints_override(ui::widget::Constraints {
                    min_size: vec2(UI_SIZE, UI_SIZE),
                    flex: vec2(1.0, 0.0),
                })
                .uniform_padding(UI_PADDING),
        ]
    }
    pub fn set_time(&mut self, time: f64, max_time: f64, ticks_per_second: f64) {
        self.time = time;
        self.max_time = max_time;
        self.ticks_per_second = ticks_per_second;
    }
    pub fn change(&self) -> Option<f64> {
        if self.captured() {
            Some(self.time)
        } else {
            None
        }
    }
}
