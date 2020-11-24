use super::*;

pub struct BackgroundGameProcessor<G: Game> {
    processor: GameProcessor<G>,
    tick_handler: Box<dyn FnMut(&G, Vec<G::Event>) + Send>,
    debug_interface: Option<DebugInterface<G>>,
}

impl<G: Game> BackgroundGameProcessor<G> {
    pub fn new(
        processor: GameProcessor<G>,
        tick_handler: impl FnMut(&G, Vec<G::Event>) + Send + 'static,
        debug_interface: Option<DebugInterface<G>>,
    ) -> Self {
        Self {
            processor,
            tick_handler: Box::new(tick_handler),
            debug_interface,
        }
    }
    pub fn proceed(&mut self, max_ticks: usize) {
        for _ in 0..max_ticks {
            if !self.processor.finished() {
                let events = self.processor.process_tick(self.debug_interface.as_ref());
                (self.tick_handler)(&self.processor.game(), events);
            }
        }
        if let Some(debug_interface) = &self.debug_interface {
            self.processor.debug_update(debug_interface);
        }
    }
    pub fn player_count(&self) -> usize {
        self.processor.player_count()
    }
}
