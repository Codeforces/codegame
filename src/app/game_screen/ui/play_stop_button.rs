use super::*;

pub struct PlayStopButton {
    button: ui::TextureButton,
    paused: Rc<Cell<bool>>,
    play_texture: Rc<ugli::Texture>,
    pause_texture: Rc<ugli::Texture>,
    last_paused: bool,
}

impl PlayStopButton {
    pub fn new(context: &Rc<Geng>, theme: &Rc<ui::Theme>, paused: &Rc<Cell<bool>>) -> Self {
        let play_texture = Rc::new(Self::create_play_texture(context));
        let pause_texture = Rc::new(Self::create_pause_texture(context));
        let button = ui::TextureButton::new(context, theme, &play_texture);
        Self {
            button,
            paused: paused.clone(),
            play_texture,
            pause_texture,
            last_paused: true,
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        let paused = &self.paused;
        if paused.get() != self.last_paused {
            self.last_paused = paused.get();
            self.button.swap(if paused.get() {
                &self.play_texture
            } else {
                &self.pause_texture
            });
        }
        self.button
            .ui(Box::new(move || paused.set(!paused.get())))
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
