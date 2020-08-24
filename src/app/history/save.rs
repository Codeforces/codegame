use super::*;

impl<G: Game, T: RendererData<G>> History<G, T> {
    pub fn save(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        let shared_state = self.shared_state.lock().unwrap();
        let mut entries = shared_state.entries.iter();
        let mut current_state = match entries.next().unwrap() {
            Entry::Full(state) => state.clone(),
            Entry::Delta(_) => panic!("First entry must be full data"),
        };
        current_state.game.write_to(&mut writer)?;
        for (entry, events) in entries.zip(shared_state.events.iter()) {
            let prev_state = current_state.clone();
            match entry {
                Entry::Full(state) => current_state = state.clone(),
                Entry::Delta(delta) => current_state.update(delta),
            };
            events.write_to(&mut writer)?;
            prev_state
                .game
                .diff(&current_state.game)
                .write_to(&mut writer)?;
        }
        Ok(())
    }
}
