use super::*;

impl<G: Game, T: RendererData<G>> History<G, T> {
    pub fn save(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        let shared_state = self.shared_state.lock().unwrap();
        let mut entries = shared_state.game.entries.iter();
        let mut current_state = match entries.next().unwrap() {
            DiffEntry::Value(state) => state.clone(),
            DiffEntry::Delta(_) => panic!("First entry must be value, not diff"),
        };
        current_state.write_to(writer)?;
        for (entry, events) in entries.zip(shared_state.events.iter()) {
            let prev_state = current_state.clone();
            match entry {
                DiffEntry::Value(state) => current_state = state.clone(),
                DiffEntry::Delta(delta) => current_state.update(delta),
            };
            events.write_to(writer)?;
            prev_state.diff(&current_state).write_to(writer)?;
        }
        Ok(())
    }
}
