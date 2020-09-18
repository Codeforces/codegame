use super::*;

pub struct FullscreenButton {
    theme: Rc<ui::Theme>,
    button: ui::Button,
    texture: ugli::Texture,
}

impl FullscreenButton {
    pub fn new(theme: &Rc<ui::Theme>) -> Self {
        Self {
            theme: theme.clone(),
            button: ui::Button::new(),
            texture: Self::create_texture(theme.geng()),
        }
    }
    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        let geng = self.theme.geng().clone();
        ui::Button::texture(
            (&mut self.button).on_click(move || geng.window().toggle_fullscreen()),
            &self.texture,
            &self.theme,
        )
        .fixed_size(vec2(UI_SIZE, UI_SIZE))
        .uniform_padding(UI_PADDING)
    }
    fn create_texture(geng: &Rc<Geng>) -> ugli::Texture {
        create_texture(geng, |framebuffer| {
            const GAP: f32 = 4.0;
            const WIDTH: f32 = 4.0;
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 0.0), vec2(16.0 - GAP, WIDTH)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 0.0), vec2(WIDTH, 16.0 - GAP)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 0.0), vec2(16.0 + GAP, WIDTH)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 0.0), vec2(32.0 - WIDTH, 16.0 - GAP)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 32.0), vec2(16.0 + GAP, 32.0 - WIDTH)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(32.0, 32.0), vec2(32.0 - WIDTH, 16.0 + GAP)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 32.0), vec2(16.0 - GAP, 32.0 - WIDTH)),
                Color::WHITE,
            );
            geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(vec2(0.0, 32.0), vec2(WIDTH, 16.0 + GAP)),
                Color::WHITE,
            );
        })
    }
}
