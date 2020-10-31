use super::*;

pub struct DebugInterface<G: Game> {
    pub(crate) debug_data_handler: Box<dyn Fn(usize, G::DebugData) + Send>,
}

impl<G: Game> DebugInterface<G> {
    pub fn for_player(&self, player_index: usize) -> PlayerDebugInterface<G> {
        PlayerDebugInterface {
            player_index,
            debug_interface: self,
        }
    }
}

pub struct PlayerDebugInterface<'a, G: Game> {
    player_index: usize,
    debug_interface: &'a DebugInterface<G>,
}

impl<G: Game> PlayerDebugInterface<'_, G> {
    pub fn send(&self, debug_data: G::DebugData) {
        (self.debug_interface.debug_data_handler)(self.player_index, debug_data);
    }
}
