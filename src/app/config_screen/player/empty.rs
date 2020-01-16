use super::*;

#[derive(Clone)]
pub struct EmptyPlayerConfig {
    geng: Rc<Geng>,
    theme: Rc<ui::Theme>,
}

impl EmptyPlayerConfig {
    pub fn new(geng: &Rc<Geng>, theme: &Rc<ui::Theme>) -> Self {
        Self {
            geng: geng.clone(),
            theme: theme.clone(),
        }
    }
    pub fn constructor<G: Game>(
        geng: &Rc<Geng>,
        theme: &Rc<ui::Theme>,
    ) -> Box<dyn Fn() -> Box<dyn PlayerConfig<G>>>
    where
        G::Action: Default,
    {
        let geng = geng.clone();
        let theme = theme.clone();
        Box::new(move || Box::new(Self::new(&geng, &theme)))
    }
}

impl<G: Game> PlayerConfig<G> for EmptyPlayerConfig
where
    G::Action: Default,
{
    fn name(&self) -> &str {
        translate("empty")
    }
    fn ui<'a>(&'a mut self) -> Box<dyn ui::Widget + 'a> {
        use ui::*;
        let ui = ui::text(
            translate("Does nothing"),
            &self.theme.font,
            16.0,
            Color::GRAY,
        )
        .align(vec2(0.5, 1.0));
        Box::new(ui)
    }
    fn ready(&mut self) -> bool {
        true
    }
    fn get(&mut self) -> Box<dyn Player<G>> {
        Box::new(EmptyPlayer)
    }
    fn to_options(&self) -> G::PlayerOptions {
        G::PlayerOptions::from(EmptyPlayerOptions)
    }
}
