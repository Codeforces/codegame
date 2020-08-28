use super::*;

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct BackgroundGameProcessor<G: Game> {
    ticks_to_process: Arc<AtomicUsize>,
    thread: std::thread::JoinHandle<()>,
    phantom_data: PhantomData<G>,
}

impl<G: Game> BackgroundGameProcessor<G> {
    pub fn new(
        mut processor: GameProcessor<G>,
        mut tick_handler: impl FnMut(&G, Vec<G::Event>) + Send + 'static,
        client_data_handler: Option<impl Fn(usize, G::ClientData) + Send + 'static>,
    ) -> Self {
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
                            let events = processor
                                .process_tick(client_data_handler.as_ref().map(|f| f as _));
                            tick_handler(processor.game(), events);
                            ticks_to_process.fetch_min(ticks - 1, Ordering::SeqCst);
                            break;
                        }
                        std::thread::park();
                    }
                }
            }
        });
        Self {
            ticks_to_process,
            thread,
            phantom_data: PhantomData,
        }
    }
    pub fn proceed(&mut self, max_ticks: usize) {
        self.ticks_to_process.store(max_ticks, Ordering::SeqCst);
        self.thread.thread().unpark();
    }
}
