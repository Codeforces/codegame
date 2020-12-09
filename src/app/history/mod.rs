use super::*;

#[cfg(not(target_arch = "wasm32"))]
mod load_native;
#[cfg(target_arch = "wasm32")]
mod load_web;
mod save;

enum DiffEntry<T: Diff> {
    Value(T),
    Delta(T::Delta),
}

struct DiffHistory<T: Diff> {
    entries: Vec<DiffEntry<T>>,
    last: T,
    last_deltas_size: u64,
}

impl<T: Diff> DiffHistory<T> {
    fn new(initial: T) -> Self {
        let last = initial.clone();
        Self {
            entries: vec![DiffEntry::Value(initial)],
            last,
            last_deltas_size: 0,
        }
    }
    fn push(&mut self, new_value: T) {
        let prev = mem::replace(&mut self.last, new_value);
        let delta = prev.diff(&self.last);
        self.last_deltas_size += bincode::serialized_size(&delta).unwrap();
        if self.last_deltas_size > bincode::serialized_size(&self.last).unwrap() {
            self.entries.push(DiffEntry::Value(self.last.clone()));
            self.last_deltas_size = 0;
        } else {
            self.entries.push(DiffEntry::Delta(delta));
        }
    }
    fn push_mut<F: FnOnce(&mut T)>(&mut self, f: F) {
        let mut new_value = self.last.clone();
        f(&mut new_value);
        self.push(new_value);
    }
    fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Clone)]
struct HistorySnapshot<T: Diff> {
    value: T,
    tick: usize,
}

impl<T: Diff> HistorySnapshot<T> {
    pub fn new(history: &DiffHistory<T>) -> Self {
        Self {
            value: match &history.entries[0] {
                DiffEntry::Value(value) => value.clone(),
                _ => unreachable!(),
            },
            tick: 0,
        }
    }
    pub fn go_to(&mut self, tick: usize, history: &DiffHistory<T>) {
        let last_full = history.entries[..=tick]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(tick, entry)| match entry {
                DiffEntry::Value(value) => Some(Self {
                    value: value.clone(),
                    tick,
                }),
                DiffEntry::Delta(_) => None,
            })
            .expect("Didn't find full entry in history");
        if tick < self.tick || self.tick < last_full.tick {
            *self = last_full;
        }
        if tick > self.tick {
            for entry in &history.entries[self.tick + 1..=tick] {
                match entry {
                    DiffEntry::Value(value) => self.value = value.clone(),
                    DiffEntry::Delta(delta) => self.value.update(delta),
                }
            }
        }
        self.tick = tick;
    }
}

struct Window<T> {
    prev: Option<T>,
    current: T,
}

impl<T: Diff> Window<HistorySnapshot<T>> {
    pub fn new(history: &DiffHistory<T>) -> Self {
        Self {
            prev: None,
            current: {
                let current = HistorySnapshot::new(history);
                assert_eq!(current.tick, 0);
                current
            },
        }
    }
    pub fn go_to(&mut self, tick: usize, history: &DiffHistory<T>) {
        if tick == 0 {
            self.prev = None;
        } else {
            if let Some(precomputed) = &mut self.prev {
                precomputed.go_to(tick - 1, history);
            } else {
                let mut prev = self.current.clone();
                prev.go_to(tick - 1, history);
                self.prev = Some(prev);
            }
        }
        self.current.go_to(tick, history);
    }
}

#[derive(Debug, Clone)]
pub struct DebugDataStorage<G: Game> {
    queued: Vec<G::DebugData>,
    cleared: bool,
    current: Vec<G::DebugData>,
    ready: bool,
    auto_flush: bool,
}

impl<'a, G: Game> IntoIterator for &'a DebugDataStorage<G> {
    type Item = &'a G::DebugData;
    type IntoIter = <&'a Vec<G::DebugData> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.current.iter()
    }
}

impl<G: Game> DebugDataStorage<G> {
    fn new() -> Self {
        Self {
            queued: Vec::new(),
            cleared: false,
            current: Vec::new(),
            auto_flush: true,
            ready: false,
        }
    }
    fn handle(&mut self, command: DebugCommand<G>) {
        match command {
            DebugCommand::Add { data } => self.queued.push(data),
            DebugCommand::Clear => {
                self.cleared = true;
                self.queued.clear();
            }
            DebugCommand::SetAutoFlush { enable } => self.auto_flush = enable,
            DebugCommand::Flush => self.flush(),
        }
        if self.auto_flush {
            self.flush();
        }
    }
    fn flush(&mut self) {
        if self.cleared {
            self.cleared = false;
            self.current.clear();
        }
        self.current.extend(self.queued.drain(..));
        self.ready = true;
    }
}

struct HistorySharedState<G: Game, T: RendererData<G>> {
    game: DiffHistory<G>,
    renderer_data: DiffHistory<T>,
    global_debug_data: HashMap<usize, DebugDataStorage<G>>,
    last_debug_data: HashMap<usize, DebugDataStorage<G>>,
    debug_data: Vec<Arc<HashMap<usize, DebugDataStorage<G>>>>,
    events: Vec<Arc<Vec<G::Event>>>,
}

impl<G: Game, T: RendererData<G>> HistorySharedState<G, T> {
    fn new(initial_game: G) -> Self {
        let initial_renderer_data = T::new(&initial_game);
        Self {
            game: DiffHistory::new(initial_game),
            renderer_data: DiffHistory::new(initial_renderer_data),
            last_debug_data: HashMap::new(),
            global_debug_data: HashMap::new(),
            debug_data: Vec::new(),
            events: Vec::new(),
        }
    }
    fn push(&mut self, game: G, events: Vec<G::Event>) {
        let prev_game = &self.game.last;
        self.renderer_data
            .push_mut(|data| RendererData::update(data, &events, &prev_game, &game));
        self.game.push(game);
        self.events.push(Arc::new(events));
        self.debug_data.push(Arc::new(mem::replace(
            &mut self.last_debug_data,
            HashMap::new(),
        )));
    }
    fn handle_debug_command(
        &mut self,
        player_index: usize,
        global: bool,
        command: DebugCommand<G>,
    ) {
        let data = if global {
            &mut self.global_debug_data
        } else {
            &mut self.last_debug_data
        };
        if !data.contains_key(&player_index) {
            data.insert(player_index, DebugDataStorage::new());
        }
        let player_data = data.get_mut(&player_index).unwrap();
        player_data.handle(command);
    }
    fn len(&self) -> usize {
        self.game.len()
    }
}

pub struct History<G: Game, T: RendererData<G>> {
    shared_state: Arc<Mutex<HistorySharedState<G, T>>>,
    game: Window<HistorySnapshot<G>>,
    renderer_data: Window<HistorySnapshot<T>>,
    debug_data: Window<Arc<HashMap<usize, DebugDataStorage<G>>>>,
    debug_data_timer: Timer,
    prev_events: Arc<Vec<G::Event>>,
    current_tick_time: f64,
}

impl<G: Game, T: RendererData<G>> History<G, T> {
    pub fn new(initial_game_state: G) -> Self {
        let shared_state = HistorySharedState::new(initial_game_state);
        let game = Window::new(&shared_state.game);
        let renderer_data = Window::new(&shared_state.renderer_data);
        let debug_data = Window {
            prev: None,
            current: Arc::new(shared_state.last_debug_data.clone()),
        };
        let prev_events = shared_state
            .events
            .last()
            .cloned()
            .unwrap_or(Arc::new(Vec::new()));
        let current_tick_time = (shared_state.len() - 1) as f64;
        Self {
            shared_state: Arc::new(Mutex::new(shared_state)),
            game,
            renderer_data,
            debug_data,
            debug_data_timer: Timer::new(),
            prev_events,
            current_tick_time,
        }
    }
    pub fn current_state(&self) -> RenderState<G, T> {
        RenderState {
            current: CurrentRenderState {
                game: &self.game.current.value,
                renderer_data: &self.renderer_data.current.value,
                debug_data: &self.debug_data.current,
            },
            prev: match (
                &self.game.prev,
                &self.renderer_data.prev,
                &self.debug_data.prev,
            ) {
                (Some(game), Some(renderer_data), Some(debug_data)) => Some(CurrentRenderState {
                    game: &game.value,
                    renderer_data: &renderer_data.value,
                    debug_data,
                }),
                _ => None,
            },
            global_debug_data: self.shared_state.lock().unwrap().global_debug_data.clone(),
            t: self.current_tick_time + 1.0 - self.current_tick_time.ceil(),
            prev_events: &self.prev_events,
        }
    }
    pub fn len(&self) -> usize {
        self.shared_state.lock().unwrap().len()
    }
    pub fn go_to(
        &mut self,
        tick_time: f64,
        collect_events: bool,
    ) -> Box<dyn Iterator<Item = G::Event>> {
        let shared_state = self.shared_state.lock().unwrap();
        let tick_time = tick_time.min((shared_state.len() - 1) as f64);
        let tick = tick_time.ceil() as usize;

        let mut events = Vec::new();
        if collect_events && tick > self.game.current.tick {
            for tick in self.game.current.tick..tick {
                events.push(shared_state.events[tick].clone());
            }
        }

        if tick != self.game.current.tick {
            self.debug_data_timer = Timer::new();
        }
        if let Some(data) = shared_state.debug_data.get(tick) {
            self.debug_data = Window {
                current: data.clone(),
                prev: if tick > 0 {
                    Some(shared_state.debug_data[tick - 1].clone())
                } else {
                    None
                },
            };
        } else {
            if tick > 0
                && self.debug_data_timer.elapsed() < 0.5
                && shared_state
                    .last_debug_data
                    .values()
                    .all(|debug_data| !debug_data.ready)
            {
                self.debug_data = Window {
                    current: shared_state.debug_data[tick - 1].clone(),
                    prev: Some(shared_state.debug_data[tick - min(tick, 2)].clone()),
                };
            } else {
                self.debug_data = Window {
                    current: Arc::new(shared_state.last_debug_data.clone()),
                    prev: if tick > 0 {
                        Some(shared_state.debug_data[tick - 1].clone())
                    } else {
                        None
                    },
                }
            }
        }

        self.game.go_to(tick, &shared_state.game);
        self.renderer_data.go_to(tick, &shared_state.renderer_data);
        self.prev_events = if tick > 0 {
            shared_state.events[tick - 1].clone()
        } else {
            Arc::new(Vec::new())
        };
        self.current_tick_time = tick_time;
        Box::new(
            events
                .into_iter()
                .flat_map(move |events| (0..events.len()).map(move |i| events[i].clone())),
        )
    }
    pub fn tick_handler(&self) -> impl FnMut(&G, Vec<G::Event>) + Send + 'static {
        let shared_state = self.shared_state.clone();
        move |game: &G, events: Vec<G::Event>| {
            shared_state.lock().unwrap().push(game.clone(), events);
        }
    }
    pub fn debug_command_handler(&self) -> impl Fn(usize, bool, DebugCommand<G>) + Send + 'static {
        let shared_state = self.shared_state.clone();
        move |player_index, global, command| {
            shared_state
                .lock()
                .unwrap()
                .handle_debug_command(player_index, global, command);
        }
    }
}
