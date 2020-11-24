use super::*;

/// Debug commands that can be sent while debugging with the app
#[trans_doc = "ru:Команды могут быть отправлены приложению для помощи в отладке"]
#[derive(Serialize, Deserialize, Trans)]
#[trans(no_generics_in_name)]
pub enum DebugCommand<G: Game> {
    /// Add debug data to current tick
    #[trans_doc = "ru:Добавить отладочные данные в текущий тик"]
    Add {
        /// Data to add
        #[trans_doc = "ru:Данные для добавления"]
        data: G::DebugData,
    },
    /// Clear current tick's debug data
    #[trans_doc = "ru:Очистить отладочные данные текущего тика"]
    Clear,
}

pub struct DebugInterface<G: Game> {
    pub(crate) debug_command_handler: Box<dyn Fn(usize, DebugCommand<G>) + Send>,
    pub(crate) debug_state: Box<dyn Fn(usize) -> G::DebugState + Send>,
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
    pub fn send(&self, command: DebugCommand<G>) {
        (self.debug_interface.debug_command_handler)(self.player_index, command);
    }
    pub fn state(&self) -> G::DebugState {
        (self.debug_interface.debug_state)(self.player_index)
    }
}
