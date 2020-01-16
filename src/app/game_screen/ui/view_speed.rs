use super::*;

pub struct ViewSpeedControl {
    context: Rc<Geng>,
    core: ui::WidgetCore,
    theme: Rc<ui::Theme>,
    slider: ui::Slider,
    value: Rc<Cell<(f64, f64)>>,
    value_text: String,
}

impl ViewSpeedControl {
    pub fn new(context: &Rc<Geng>, theme: &Rc<ui::Theme>, value: &Rc<Cell<(f64, f64)>>) -> Self {
        Self {
            context: context.clone(),
            core: ui::WidgetCore::new(),
            theme: theme.clone(),
            slider: ui::Slider::new(context, theme),
            value: value.clone(),
            value_text: String::new(),
        }
    }

    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        let value = &self.value;
        let show_slider = self.core.hovered() || self.core.captured();
        let slider_used = self.slider.hovered() || self.slider.captured();
        // if needs to be updated
        if true {
            self.value_text.clear();
            use std::fmt::Write;
            write!(
                &mut self.value_text,
                "{}% ({} TPS)",
                (value.get().0 * 100.0) as i32,
                (value.get().0 * value.get().1) as i32
            )
            .unwrap();
        }
        ui::stack![
            ui::column(vec![
                if show_slider {
                    Box::new(self.slider.ui(
                        value.get().0,
                        0.0..=2.0,
                        Box::new(move |new_value| value.set((new_value, value.get().1))),
                    ))
                } else {
                    Box::new(
                        text(
                            translate("view speed"),
                            self.context.default_font(),
                            UI_SIZE as f32 / 2.0,
                            self.theme.color,
                        )
                        .maintain_aspect(vec2(0.5, 0.5)),
                    )
                },
                Box::new(
                    text(
                        &self.value_text,
                        self.context.default_font(),
                        UI_SIZE as f32 / 2.0,
                        if slider_used {
                            self.theme.hover_color
                        } else {
                            self.theme.color
                        }
                    )
                    .maintain_aspect(vec2(0.5, 0.5)),
                )
            ]),
            ui::control_panel(&mut self.core),
        ]
        .fixed_size(vec2(UI_SIZE * 3.0, UI_SIZE))
        .uniform_padding(UI_PADDING)
    }
}
