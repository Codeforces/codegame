use super::*;

pub struct FullscreenButton {
    context: Rc<Geng>,
    button: ui::TextureButton,
}

impl FullscreenButton {
    pub fn new(context: &Rc<Geng>, theme: &Rc<ui::Theme>) -> Self {
        Self {
            context: context.clone(),
            button: ui::TextureButton::new(context, theme, &Rc::new(Self::create_texture(context))),
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        let context = &self.context;
        self.button
            .ui(Box::new(move || context.window().toggle_fullscreen()))
            .fixed_size(vec2(UI_SIZE, UI_SIZE))
            .uniform_padding(UI_PADDING)
    }
    fn create_texture(context: &Rc<Geng>) -> ugli::Texture {
        create_texture(context, |framebuffer| {
            const GAP: f32 = 4.0;
            const WIDTH: f32 = 4.0;
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 0.0), vec2(16.0 - GAP, WIDTH)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 0.0), vec2(WIDTH, 16.0 - GAP)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 0.0), vec2(16.0 + GAP, WIDTH)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 0.0), vec2(32.0 - WIDTH, 16.0 - GAP)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 32.0), vec2(16.0 + GAP, 32.0 - WIDTH)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 32.0), vec2(32.0 - WIDTH, 16.0 + GAP)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 32.0), vec2(16.0 - GAP, 32.0 - WIDTH)),
                Color::WHITE,
            );
            context.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 32.0), vec2(WIDTH, 16.0 + GAP)),
                Color::WHITE,
            );
        })
    }
}
