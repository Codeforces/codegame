use super::*;

pub struct VolumeControl {
    core: ui::WidgetCore,
    theme: Rc<ui::Theme>,
    slider: ui::Slider,
    value: Rc<Cell<f64>>,
    value_text: String,
}

impl VolumeControl {
    pub fn new(theme: &Rc<ui::Theme>, value: &Rc<Cell<f64>>) -> Self {
        Self {
            core: ui::WidgetCore::new(),
            theme: theme.clone(),
            slider: ui::Slider::new(theme),
            value: value.clone(),
            value_text: String::new(),
        }
    }

    // TODO should not be dead code
    #[allow(dead_code)]
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        let value = &self.value;
        let show_slider = self.core.hovered() || self.core.captured();
        let slider_used = self.slider.hovered() || self.slider.captured();
        // if needs to be updated
        if true {
            self.value_text.clear();
            use std::fmt::Write;
            write!(&mut self.value_text, "{}%", (value.get() * 100.0) as i32).unwrap();
        }
        ui::stack![
            ui::column(vec![
                if show_slider {
                    Box::new(
                        self.slider
                            .ui(
                                value.get(),
                                0.0..=1.0,
                                Box::new(move |new_value| value.set(new_value)),
                            )
                            .fixed_size(vec2(UI_SIZE * 2.0, UI_SIZE / 2.0)),
                    )
                } else {
                    Box::new(
                        ui::Text::new(
                            translate("volume"),
                            &self.theme.font,
                            UI_SIZE as f32 / 2.0,
                            self.theme.usable_color,
                        )
                        .maintain_aspect(vec2(0.5, 0.5)),
                    )
                },
                Box::new(
                    ui::Text::new(
                        &self.value_text,
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
        .fixed_size(vec2(UI_SIZE * 2.0, UI_SIZE))
        .uniform_padding(UI_PADDING)
    }
}
