use super::*;
use geng::ui;

mod fullscreen_button;
mod play_stop_button;
mod timeline;
mod view_speed;
mod volume;

use fullscreen_button::*;
use play_stop_button::*;
use timeline::*;
use view_speed::*;
use volume::*;

const UI_SIZE: f64 = 32.0;
const UI_PADDING: f64 = 8.0;

fn create_texture(context: &Rc<Geng>, f: impl FnOnce(&mut ugli::Framebuffer)) -> ugli::Texture {
    let mut texture = ugli::Texture::new_uninitialized(context.ugli(), vec2(32, 32));
    {
        let mut framebuffer = ugli::Framebuffer::new(
            context.ugli(),
            ugli::ColorAttachment::Texture(&mut texture),
            ugli::DepthAttachment::None,
        );
        ugli::clear(
            &mut framebuffer,
            Some(Color::rgba(1.0, 1.0, 1.0, 0.0)),
            None,
        );
        f(&mut framebuffer);
    }
    texture
}

pub struct UI {
    play_stop_button: PlayStopButton,
    fullscreen_button: FullscreenButton,
    timeline: Timeline,
    view_speed: ViewSpeedControl,
    volume: VolumeControl,
}

impl UI {
    pub fn new(
        geng: &Rc<Geng>,
        paused: &Rc<Cell<bool>>,
        view_speed: &Rc<Cell<(f64, f64)>>,
        volume: &Rc<Cell<f64>>,
    ) -> Self {
        let theme = Rc::new(ui::Theme::default(geng));
        let theme = &theme;
        Self {
            play_stop_button: PlayStopButton::new(geng, theme, paused),
            fullscreen_button: FullscreenButton::new(geng, theme),
            timeline: Timeline::new(geng, theme),
            view_speed: ViewSpeedControl::new(geng, theme, view_speed),
            volume: VolumeControl::new(geng, theme, volume),
        }
    }

    pub fn timeline_change(&self) -> Option<f64> {
        self.timeline.change()
    }

    pub fn set_time(&mut self, time: f64, max_time: f64, ticks_per_second: f64) {
        self.timeline.set_time(time, max_time, ticks_per_second);
    }

    pub fn ui<'a>(&'a mut self) -> impl ui::Widget + 'a {
        use ui::*;
        geng::ui::row![
            self.play_stop_button.ui(),
            self.timeline.ui(),
            self.view_speed.ui(),
            self.volume.ui(),
            self.fullscreen_button.ui(),
        ]
        .align(vec2(0.5, 0.0))
    }
}
