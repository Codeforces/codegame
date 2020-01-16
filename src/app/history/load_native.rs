use super::*;

impl<G: Game, R: Renderer<G>> History<G, R> {
    pub fn load(path: &str) -> impl Future<Output = Self> {
        fn load<G: Game, R: Renderer<G>>(
            path: &str,
        ) -> std::io::Result<impl Future<Output = History<G, R>>> {
            let mut reader = std::io::BufReader::new(std::fs::File::open(path)?);
            let initial_state = G::read_from(&mut reader)?;
            let history = History::<G, R>::new(&initial_state);
            let mut tick_handler = history.tick_handler();
            let mut current_state = initial_state;
            std::thread::spawn(move || {
                let mut f = move || -> std::io::Result<()> {
                    while !reader.fill_buf()?.is_empty() {
                        let events = Vec::<G::Event>::read_from(&mut reader)?;
                        let delta = G::Delta::read_from(&mut reader)?;
                        current_state.update(&delta);
                        tick_handler(&current_state, events);
                    }
                    Ok(())
                };
                f().expect("Error while reading replay")
            });
            Ok(futures::future::ready(history))
        }
        load::<G, R>(path).expect("Failed to load replay")
    }
}
