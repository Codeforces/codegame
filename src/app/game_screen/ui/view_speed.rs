use super::*;

pub fn view_speed(modifier: f64, default_tps: f64) -> f64 {
    default_tps * modifier.exp2()
}

pub struct ViewSpeedControl {
    core: ui::WidgetCore,
    theme: Rc<ui::Theme>,
    slider: ui::Slider,
    modifier: Rc<Cell<f64>>,
    text: String,
}

impl ViewSpeedControl {
    pub const MODIFIER_RANGE: RangeInclusive<f64> = -2.0..=2.0;

    pub fn new(theme: &Rc<ui::Theme>, modifier: &Rc<Cell<f64>>) -> Self {
        Self {
            core: ui::WidgetCore::new(),
            theme: theme.clone(),
            slider: ui::Slider::new(theme),
            modifier: modifier.clone(),
            text: String::new(),
        }
    }

    pub fn ui<'a>(&'a mut self, default_tps: f64) -> impl ui::Widget + 'a {
        use ui::*;
        let modifier = &self.modifier;
        let show_slider = self.core.hovered() || self.core.captured();
        let slider_used = self.slider.hovered() || self.slider.captured();
        // if needs to be updated
        if true {
            self.text.clear();
            use std::fmt::Write;
            let tps = view_speed(modifier.get(), default_tps);
            if tps < 0.95 {
                let tps10 = (tps * 10.0).round() as i32;
                write!(
                    &mut self.text,
                    "{}% ({}.{} TPS)",
                    view_speed(modifier.get(), 100.0) as i32,
                    tps10 / 10,
                    tps10 % 10,
                )
            } else {
                write!(
                    &mut self.text,
                    "{}% ({} TPS)",
                    view_speed(modifier.get(), 100.0) as i32,
                    tps.round() as i32,
                )
            }
            .unwrap();
        }
        ui::stack![
            ui::column(vec![
                if show_slider {
                    Box::new(
                        self.slider
                            .ui(
                                modifier.get(),
                                Self::MODIFIER_RANGE,
                                Box::new(move |new_value| modifier.set(new_value)),
                            )
                            .fixed_size(vec2(UI_SIZE * 3.0, UI_SIZE / 2.0)),
                    )
                } else {
                    Box::new(
                        ui::Text::new(
                            translate("view speed"),
                            &self.theme.font,
                            UI_SIZE as f32 / 2.0,
                            self.theme.usable_color,
                        )
                        .maintain_aspect(vec2(0.5, 0.5)),
                    )
                },
                Box::new(
                    ui::Text::new(
                        &self.text,
                        &self.theme.font,
                        UI_SIZE as f32 / 2.0,
                        if slider_used {
                            self.theme.hover_color
                        } else {
                            self.theme.usable_color
                        }
                    )
                    .maintain_aspect(vec2(0.5, 0.5)),
                )
            ]),
            &mut self.core,
        ]
        .fixed_size(vec2(UI_SIZE * 3.0, UI_SIZE))
        .uniform_padding(UI_PADDING)
    }
}
