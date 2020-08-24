use super::*;

pub struct BackgroundGameProcessor<G: Game> {
    processor: GameProcessor<G>,
    tick_handler: Box<dyn FnMut(&G, Vec<G::Event>) + Send>,
    client_data_handler: Option<Box<dyn Fn(usize, G::ClientData) + Send>>,
}

impl<G: Game> BackgroundGameProcessor<G> {
    pub fn new(
        processor: GameProcessor<G>,
        tick_handler: impl FnMut(&G, Vec<G::Event>) + Send + 'static,
        client_data_handler: Option<impl Fn(usize, G::ClientData) + Send + 'static>,
    ) -> Self {
        Self {
            processor,
            tick_handler: Box::new(tick_handler),
            client_data_handler: client_data_handler.map(|f| Box::new(f) as _),
        }
    }
    pub fn proceed(&mut self, mut max_ticks: usize) {
        for _ in 0..max_ticks {
            if !self.processor.finished() {
                let events = self
                    .processor
                    .process_tick(self.client_data_handler.as_ref().map(|f| f as _));
                (self.tick_handler)(&self.processor.game(), events);
            }
        }
    }
}
