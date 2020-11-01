use super::*;

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct BackgroundGameProcessor<G: Game> {
    player_count: usize,
    ticks_to_process: Arc<AtomicUsize>,
    thread: std::thread::JoinHandle<()>,
    phantom_data: PhantomData<G>,
}

impl<G: Game> BackgroundGameProcessor<G> {
    pub fn new(
        mut processor: GameProcessor<G>,
        mut tick_handler: impl FnMut(&G, Vec<G::Event>) + Send + 'static,
        debug_interface: Option<DebugInterface<G>>,
    ) -> Self {
        let player_count = processor.player_count();
        let ticks_to_process = Arc::new(AtomicUsize::new(0));
        let thread = std::thread::spawn({
            let ticks_to_process = ticks_to_process.clone();
            move || {
                while !processor.finished() {
                    loop {
                        let ticks = ticks_to_process.load(Ordering::SeqCst);
                        if ticks > 0
                            && ticks_to_process.compare_and_swap(ticks, ticks - 1, Ordering::SeqCst)
                                == ticks
                        {
                            let events = processor.process_tick(debug_interface.as_ref());
                            tick_handler(processor.game(), events);
                            ticks_to_process.fetch_min(ticks - 1, Ordering::SeqCst);
                            break;
                        }
                        if let Some(debug_interface) = &debug_interface {
                            processor.debug_update(debug_interface);
                        }
                        std::thread::park();
                    }
                }
            }
        });
        Self {
            player_count,
            ticks_to_process,
            thread,
            phantom_data: PhantomData,
        }
    }
    pub fn proceed(&mut self, max_ticks: usize) {
        self.ticks_to_process.store(max_ticks, Ordering::SeqCst);
        self.thread.thread().unpark();
    }
    pub fn player_count(&self) -> usize {
        self.player_count
    }
}
