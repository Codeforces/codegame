use super::*;

mod background;

pub use background::*;

enum State<G: Game> {
    Game {
        game: G,
        rng: Box<dyn RngCore + Send>,
    },
    Repeater {
        game: G,
        reader: Box<dyn std::io::BufRead + Send>,
        finished: bool,
    },
}

impl<G: Game> State<G> {
    fn process_turn(&mut self, actions: HashMap<usize, G::Action>) -> Vec<G::Event> {
        match self {
            Self::Game { game, rng } => game.process_turn(rng, actions),
            Self::Repeater {
                game,
                reader,
                finished,
            } => {
                assert!(!*finished);
                let events =
                    Vec::<G::Event>::read_from(&mut *reader).expect("Failed to read game log");
                let delta = G::Delta::read_from(&mut *reader).expect("Failed to read game log");
                game.update(&delta);
                self.update_finished();
                events
            }
        }
    }
    fn game(&self) -> &G {
        match self {
            Self::Game { game, .. } | Self::Repeater { game, .. } => game,
        }
    }
    fn update_finished(&mut self) {
        match self {
            Self::Game { .. } => {}
            Self::Repeater {
                reader, finished, ..
            } => {
                if reader
                    .fill_buf()
                    .expect("Failed to read game log")
                    .is_empty()
                {
                    *finished = true;
                }
            }
        }
    }
    fn finished(&self) -> bool {
        match self {
            Self::Game { game, .. } => game.finished(),
            Self::Repeater { finished, .. } => *finished,
        }
    }
}

pub struct GameProcessor<G: Game> {
    seed: Option<u64>,
    state: State<G>,
    players: Vec<Option<Box<dyn Player<G>>>>,
    player_comments: Vec<Option<String>>,
    ticks_processed: usize,
    tick_handler: Option<Box<dyn FnMut(Option<&Vec<G::Event>>, &G) + Send>>,
    results_handler: Option<Box<dyn FnOnce(FullResults<G>) + Send>>,
}

impl<G: Game + 'static> GameProcessor<G> {
    pub fn new_full(full_options: FullOptions<G>, player_extra_data: &G::PlayerExtraData) -> Self {
        Self::new(
            full_options.seed,
            full_options.options_preset.into(),
            futures::executor::block_on(futures::future::join_all(
                full_options
                    .players
                    .iter()
                    .map(|options| options.get(player_extra_data)),
            ))
            .into_iter()
            .map(|result| match result {
                Ok(player) => player,
                Err(e) => Box::new(ErroredPlayer(e.to_string())),
            })
            .collect(),
        )
    }
    pub fn new(seed: Option<u64>, options: G::Options, players: Vec<Box<dyn Player<G>>>) -> Self {
        let seed = seed.unwrap_or_else(|| global_rng().gen());
        let mut rng = Box::new(<rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(
            seed,
        ));
        let game = G::init(&mut rng, players.len(), options);
        let player_comments = vec![None; players.len()];
        Self {
            seed: Some(seed),
            state: State::Game { game, rng },
            players: players.into_iter().map(|player| Some(player)).collect(),
            player_comments,
            ticks_processed: 0,
            tick_handler: None,
            results_handler: None,
        }
    }
    pub fn repeat_full(
        full_options: FullOptions<G>,
        player_extra_data: &G::PlayerExtraData,
        reader: impl std::io::Read + Send + 'static,
    ) -> Self {
        Self::repeat(
            reader,
            futures::executor::block_on(futures::future::join_all(
                full_options
                    .players
                    .iter()
                    .map(|options| options.get(player_extra_data)),
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
        let mut reader = std::io::BufReader::new(reader);
        let game = G::read_from(&mut reader).expect("Failed to read game log");
        let mut state = State::Repeater {
            game,
            reader: Box::new(reader),
            finished: false,
        };
        state.update_finished();
        let player_comments = vec![None; players.len()];
        Self {
            seed: None,
            state,
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
        handler(None, self.state.game());
        self.tick_handler = Some(handler);
    }
    pub fn set_results_handler(&mut self, handler: Box<dyn FnOnce(FullResults<G>) + Send>) {
        self.results_handler = Some(handler);
    }

    pub(crate) fn process_tick(
        &mut self,
        client_data_handler: Option<&dyn Fn(usize, G::ClientData)>,
    ) -> Vec<G::Event> {
        assert!(!self.finished());
        let views: Vec<_> = (0..self.players.len())
            .map(|index| self.state.game().player_view(index))
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
                            client_data_handler
                                .map(move |f| (move |client_data| f(index, client_data)))
                                .as_ref()
                                .map(|f| f as _),
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
        let events = self.state.process_turn(actions);
        if let Some(handler) = &mut self.tick_handler {
            handler(Some(&events), self.state.game());
        }
        if self.finished() {
            let results = self.state.game().results();
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

    pub fn run(mut self) {
        while !self.finished() {
            self.process_tick(None);
        }
    }

    pub fn game(&self) -> &G {
        self.state.game()
    }

    pub fn finished(&self) -> bool {
        self.state.finished()
    }
}
