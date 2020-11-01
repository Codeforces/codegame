use super::*;

mod background;
#[path = "strategy/mod.rs"]
pub mod processor_strategy;

pub use background::*;
pub use processor_strategy::GameProcessorStrategy;

pub struct GameProcessor<G: Game> {
    seed: Option<u64>,
    strategy: Box<dyn GameProcessorStrategy<G>>,
    players: Vec<Option<Box<dyn Player<G>>>>,
    player_comments: Vec<Option<String>>,
    ticks_processed: usize,
    tick_handler: Option<Box<dyn FnMut(Option<&Vec<G::Event>>, &G) + Send>>,
    results_handler: Option<Box<dyn FnOnce(FullResults<G>) + Send>>,
}

impl<G: Game + 'static> GameProcessor<G> {
    pub fn new_full(full_options: FullOptions<G>) -> Self {
        Self::new(
            full_options.seed,
            full_options.game.into(),
            futures::executor::block_on(futures::future::join_all(
                full_options.players.iter().map(|options| options.get()),
            ))
            .into_iter()
            .map(|result| match result {
                Ok(player) => player,
                Err(e) => Box::new(ErroredPlayer(e.to_string())),
            })
            .collect(),
        )
    }
    pub fn new(
        seed: Option<u64>,
        options: GameInitOptions<G>,
        players: Vec<Box<dyn Player<G>>>,
    ) -> Self {
        let seed = seed.unwrap_or_else(|| global_rng().gen());
        let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(seed);
        let game = match options {
            GameInitOptions::Ready(game) => game,
            GameInitOptions::New(options) => G::init(&mut rng, players.len(), options),
        };
        let player_comments = vec![None; players.len()];
        Self {
            seed: Some(seed),
            strategy: Box::new(processor_strategy::Standard::new(game, rng)),
            players: players.into_iter().map(|player| Some(player)).collect(),
            player_comments,
            ticks_processed: 0,
            tick_handler: None,
            results_handler: None,
        }
    }
    pub fn repeat_full(
        full_options: FullOptions<G>,
        reader: impl std::io::Read + Send + 'static,
    ) -> Self {
        Self::repeat(
            reader,
            futures::executor::block_on(futures::future::join_all(
                full_options.players.iter().map(|options| options.get()),
            ))
            .into_iter()
            .map(|result| match result {
                Ok(player) => player,
                Err(e) => Box::new(ErroredPlayer(e.to_string())),
            })
            .collect(),
        )
    }
    pub fn repeat(
        reader: impl std::io::Read + Send + 'static,
        players: Vec<Box<dyn Player<G>>>,
    ) -> Self {
        let player_comments = vec![None; players.len()];
        Self {
            seed: None,
            strategy: Box::new(processor_strategy::Repeat::new(Box::new(
                std::io::BufReader::new(reader),
            ))),
            players: players.into_iter().map(|player| Some(player)).collect(),
            player_comments,
            ticks_processed: 0,
            tick_handler: None,
            results_handler: None,
        }
    }

    pub fn set_tick_handler(
        &mut self,
        mut handler: Box<dyn FnMut(Option<&Vec<G::Event>>, &G) + Send>,
    ) {
        handler(None, self.strategy.game());
        self.tick_handler = Some(handler);
    }
    pub fn set_results_handler(&mut self, handler: Box<dyn FnOnce(FullResults<G>) + Send>) {
        self.results_handler = Some(handler);
    }

    pub(crate) fn debug_update(&mut self, debug_interface: &DebugInterface<G>) {
        for (index, player_cell) in self.players.iter_mut().enumerate() {
            if let Some(player) = player_cell {
                if let Err(e) = player.debug_update(
                    &self.strategy.game().player_view(index),
                    &debug_interface.for_player(index),
                ) {
                    *player_cell = None;
                    warn!("Player error: {}", e);
                    self.player_comments[index] = Some(format!("Player crashed: {}", e));
                }
            }
        }
    }

    pub(crate) fn process_tick(
        &mut self,
        debug_interface: Option<&DebugInterface<G>>,
    ) -> Vec<G::Event> {
        assert!(!self.finished());
        let views: Vec<_> = (0..self.players.len())
            .map(|index| self.strategy.game().player_view(index))
            .collect();
        let action_results = self
            .players
            .iter_mut()
            .zip(views.into_iter())
            .enumerate()
            .filter_map(|(index, (player, view))| {
                player.as_mut().map(|player| {
                    (
                        index,
                        player.get_action(
                            &view,
                            debug_interface
                                .map(|debug_interface| debug_interface.for_player(index))
                                .as_ref(),
                        ),
                    )
                })
            });
        let player_comments = &mut self.player_comments;
        let actions: HashMap<usize, G::Action> = action_results
            .filter_map(|(index, result)| {
                if let Err(e) = &result {
                    warn!("Player error: {}", e);
                    player_comments[index] = Some(format!("Player crashed: {}", e));
                }
                result.ok().map(|action| (index, action))
            })
            .collect();
        for (index, player) in self.players.iter_mut().enumerate() {
            if !actions.contains_key(&index) {
                *player = None;
            }
        }
        let events = self.strategy.process_turn(actions);
        if let Some(handler) = &mut self.tick_handler {
            handler(Some(&events), self.strategy.game());
        }
        if self.finished() {
            let results = self.strategy.game().results();
            if let Some(handler) = self.results_handler.take() {
                handler(FullResults {
                    players: self
                        .players
                        .iter()
                        .zip(self.player_comments.iter())
                        .map(|(player, comment)| PlayerResult {
                            crashed: player.is_none(),
                            comment: comment.clone(),
                        })
                        .collect(),
                    results,
                    seed: self.seed,
                });
            }
        }
        self.ticks_processed += 1;
        trace!("Processed {} ticks", self.ticks_processed);
        events
    }

    pub fn run(mut self, debug_interface: Option<&DebugInterface<G>>) {
        while !self.finished() {
            self.process_tick(debug_interface);
            if let Some(debug_interface) = debug_interface {
                self.debug_update(debug_interface);
            }
        }
    }

    pub fn game(&self) -> &G {
        self.strategy.game()
    }

    pub fn finished(&self) -> bool {
        self.strategy.finished()
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}
