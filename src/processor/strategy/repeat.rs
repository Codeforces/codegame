use super::*;

pub struct Repeat<G: Game> {
    game: G,
    reader: Box<dyn std::io::BufRead + Send>,
    finished: bool,
}

impl<G: Game> Repeat<G> {
    pub fn new(reader: impl std::io::Read + Send + 'static) -> Self {
        let mut reader = std::io::BufReader::new(reader);
        let game = G::read_from(&mut reader).expect("Failed to read game log");
        let mut result = Self {
            game,
            reader: Box::new(reader),
            finished: false,
        };
        result.update_finished();
        result
    }
    fn update_finished(&mut self) {
        self.finished = self
            .reader
            .fill_buf()
            .expect("Failed to read game log")
            .is_empty();
    }
}

impl<G: Game> GameProcessorStrategy<G> for Repeat<G> {
    fn process_turn(&mut self, _actions: HashMap<usize, G::Action>) -> Vec<G::Event> {
        assert!(!self.finished());
        let events = Vec::<G::Event>::read_from(&mut self.reader).expect("Failed to read game log");
        let delta = G::Delta::read_from(&mut self.reader).expect("Failed to read game log");
        self.update_finished();
        self.game.update(&delta);
        events
    }
    fn game(&self) -> &G {
        &self.game
    }
    fn finished(&self) -> bool {
        self.finished
    }
}
