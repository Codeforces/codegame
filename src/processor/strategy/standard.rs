use super::*;

pub struct Standard<G: Game> {
    game: G,
    rng: Box<dyn RngCore + Send>,
}

impl<G: Game> Standard<G> {
    pub fn new(game: G, rng: impl RngCore + Send + 'static) -> Self {
        Self {
            game,
            rng: Box::new(rng),
        }
    }
}

impl<G: Game> GameProcessorStrategy<G> for Standard<G> {
    fn process_turn(&mut self, actions: HashMap<usize, G::Action>) -> Vec<G::Event> {
        self.game.process_turn(&mut self.rng, actions)
    }
    fn game(&self) -> &G {
        &self.game
    }
    fn finished(&self) -> bool {
        self.game.finished()
    }
}
