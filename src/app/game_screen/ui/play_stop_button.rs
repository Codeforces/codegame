use super::*;

pub struct PlayStopButton {
    theme: Rc<ui::Theme>,
    button: ui::Button,
    paused: Rc<Cell<bool>>,
    play_texture: ugli::Texture,
    pause_texture: ugli::Texture,
}

impl PlayStopButton {
    pub fn new(theme: &Rc<ui::Theme>, paused: &Rc<Cell<bool>>) -> Self {
        Self {
            theme: theme.clone(),
            button: ui::Button::new(),
            paused: paused.clone(),
            play_texture: Self::create_play_texture(theme.geng()),
            pause_texture: Self::create_pause_texture(theme.geng()),
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        let paused = &self.paused;
        ui::Button::texture(
            (&mut self.button).on_click(move || paused.set(!paused.get())),
            if self.paused.get() {
                &self.play_texture
            } else {
                &self.pause_texture
            },
            &self.theme,
        )
        .fixed_size(vec2(UI_SIZE, UI_SIZE))
        .uniform_padding(UI_PADDING)
    }
    fn create_play_texture(context: &Rc<Geng>) -> ugli::Texture {
        create_texture(context, |framebuffer| {
            context.draw_2d().draw(
                framebuffer,
                &[vec2(0.0, 0.0), vec2(32.0, 16.0), vec2(0.0, 32.0)],
                Color::WHITE,
                ugli::DrawMode::Triangles,
            );
        })
    }
    fn create_pause_texture(context: &Rc<Geng>) -> ugli::Texture {
        create_texture(context, |framebuffer| {
            const GAP: f32 = 5.0;
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 0.0), vec2(16.0 - GAP, 32.0)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 0.0), vec2(16.0 + GAP, 32.0)),
                Color::WHITE,
            );
        })
    }
}
