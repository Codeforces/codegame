use super::Debug;

pub struct MyStrategy {}

impl MyStrategy {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_action(
        &mut self,
        player_view: &model::PlayerView,
        debug_interface: &mut DebugInterface,
    ) -> model::Action {
        todo!()
    }
    pub fn debug_update(
        &mut self,
        player_view: &model::PlayerView,
        debug_interface: &mut DebugInterface,
    ) {
    }
}
