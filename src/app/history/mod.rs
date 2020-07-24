use super::*;

#[cfg(not(target_arch = "wasm32"))]
mod load_native;
#[cfg(target_arch = "wasm32")]
mod load_web;
mod save;

#[derive(Serialize)]
struct State<G: Game, R: Renderer<G>> {
    game: G,
    last_events: Vec<G::Event>,
    extra_data: R::ExtraData,
}

impl<G: Game, R: Renderer<G>> State<G, R> {
    fn diff(&self, other: &Self) -> (G::Delta, <R::ExtraData as Diff>::Delta) {
        (
            Diff::diff(&self.game, &other.game),
            Diff::diff(&self.extra_data, &other.extra_data),
        )
    }
    fn update(&mut self, delta: &(G::Delta, <R::ExtraData as Diff>::Delta)) {
        Diff::update(&mut self.game, &delta.0);
        Diff::update(&mut self.extra_data, &delta.1);
    }
}

impl<G: Game, R: Renderer<G>> Clone for State<G, R> {
    fn clone(&self) -> Self {
        Self {
            game: self.game.clone(),
            last_events: self.last_events.clone(),
            extra_data: self.extra_data.clone(),
        }
    }
}

enum Entry<G: Game, R: Renderer<G>> {
    Full(State<G, R>),
    Delta((G::Delta, <R::ExtraData as Diff>::Delta)),
}

struct SharedState<G: Game, R: Renderer<G>> {
    entries: Vec<Entry<G, R>>,
    custom_data: Vec<Arc<HashMap<usize, Vec<G::CustomData>>>>,
    events: Vec<Vec<G::Event>>,
    last_state: State<G, R>,
    last_custom_data: HashMap<usize, Vec<G::CustomData>>,
    total_latest_delta_size: u64,
}

impl<G: Game, R: Renderer<G>> SharedState<G, R> {
    fn push(&mut self, game: &G, events: Vec<G::Event>) {
        let prev_state = self.last_state.clone();
        self.last_state.game = game.clone();
        RendererExtraData::update(&mut self.last_state.extra_data, &events, game);
        let delta = prev_state.diff(&self.last_state);
        self.total_latest_delta_size +=
            bincode::serialized_size(&delta).expect("Failed to get delta serialized size");
        if self.total_latest_delta_size
            > bincode::serialized_size(&self.last_state)
                .expect("Failed to get last state serialized size")
        {
            self.entries.push(Entry::Full(self.last_state.clone()));
            self.total_latest_delta_size = 0;
        } else {
            self.entries.push(Entry::Delta(delta));
        }
        self.events.push(events);
        self.custom_data.push(Arc::new(mem::replace(
            &mut self.last_custom_data,
            HashMap::new(),
        )));
    }
    fn push_custom_data(&mut self, player_index: usize, custom_data: G::CustomData) {
        if !self.last_custom_data.contains_key(&player_index) {
            self.last_custom_data.insert(player_index, Vec::new());
        }
        self.last_custom_data
            .get_mut(&player_index)
            .unwrap()
            .push(custom_data);
    }
    fn len(&self) -> usize {
        self.entries.len()
    }
}

pub struct History<G: Game, R: Renderer<G>> {
    shared_state: Arc<Mutex<SharedState<G, R>>>,
    current_tick_timer: Timer,
    current_tick: usize,
    current_state: State<G, R>,
    current_custom_data: Arc<HashMap<usize, Vec<G::CustomData>>>,
}

impl<G: Game, R: Renderer<G>> History<G, R> {
    pub fn new(initial_game_state: &G) -> Self {
        let initial_extra_data = R::ExtraData::new(initial_game_state);
        let initial_state = State {
            game: initial_game_state.clone(),
            last_events: Vec::new(),
            extra_data: initial_extra_data,
        };
        Self {
            shared_state: Arc::new(Mutex::new(SharedState {
                entries: vec![Entry::Full(initial_state.clone())],
                events: Vec::new(),
                last_state: initial_state.clone(),
                custom_data: Vec::new(),
                last_custom_data: HashMap::new(),
                total_latest_delta_size: 0,
            })),
            current_state: initial_state.clone(),
            current_custom_data: Arc::new(HashMap::new()),
            current_tick_timer: Timer::new(),
            current_tick: 0,
        }
    }
    pub fn current_state(
        &self,
    ) -> (
        &G,
        &Vec<G::Event>,
        &R::ExtraData,
        &HashMap<usize, Vec<G::CustomData>>,
    ) {
        (
            &self.current_state.game,
            &self.current_state.last_events,
            &self.current_state.extra_data,
            &self.current_custom_data,
        )
    }
    pub fn len(&self) -> usize {
        self.shared_state.lock().unwrap().len()
    }
    pub fn go_to(
        &mut self,
        tick: usize,
        collect_events: bool,
    ) -> Box<dyn Iterator<Item = G::Event>> {
        let shared_state = self.shared_state.lock().unwrap();
        let tick = tick.min(shared_state.len() - 1);
        let mut prev_tick = self.current_tick;
        let mut events = Vec::new();
        if collect_events && tick > prev_tick {
            for tick in prev_tick..tick {
                events.extend(shared_state.events[tick].iter().cloned());
            }
        }
        let (last_full_tick, last_full_state) = shared_state.entries[..=tick]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(tick, entry)| match entry {
                Entry::Full(game) => Some((tick, game)),
                Entry::Delta(_) => None,
            })
            .expect("Didn't find full game entry in history");
        if tick < prev_tick || prev_tick < last_full_tick {
            self.current_state = last_full_state.clone();
            prev_tick = last_full_tick;
        }
        if tick > prev_tick {
            for entry in &shared_state.entries[prev_tick + 1..=tick] {
                match entry {
                    Entry::Full(state) => self.current_state = state.clone(),
                    Entry::Delta(delta) => self.current_state.update(delta),
                }
            }
        }
        self.current_state.last_events = if tick == 0 {
            Vec::new()
        } else {
            shared_state.events[tick - 1].clone()
        };
        if tick != self.current_tick {
            self.current_tick_timer = Timer::new();
        }
        if let Some(data) = shared_state.custom_data.get(tick) {
            self.current_custom_data = data.clone();
        } else {
            if tick > 0 && self.current_tick_timer.elapsed() < 0.5 {
                self.current_custom_data = shared_state.custom_data[tick - 1].clone();
            } else {
                self.current_custom_data = Arc::new(shared_state.last_custom_data.clone());
            }
        }
        self.current_tick = tick;
        Box::new(events.into_iter())
    }
    pub fn tick_handler(&self) -> impl FnMut(&G, Vec<G::Event>) + Send + 'static {
        let shared_state = self.shared_state.clone();
        move |game: &G, events: Vec<G::Event>| {
            shared_state.lock().unwrap().push(game, events);
        }
    }
    pub fn custom_data_handler(&self) -> impl Fn(usize, G::CustomData) + Send + 'static {
        let shared_state = self.shared_state.clone();
        move |player_index, custom_data| {
            shared_state
                .lock()
                .unwrap()
                .push_custom_data(player_index, custom_data);
        }
    }
}
