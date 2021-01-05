use super::*;

mod stream;
mod tcp;

pub use stream::*;
pub use tcp::*;

#[derive(Debug, thiserror::Error)]
pub enum PlayerError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

pub trait Player<G: Game>: Send {
    fn get_action(
        &mut self,
        player_view: &G::PlayerView,
        debug_interface: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError>;
    fn debug_update(
        &mut self,
        player_view: &G::PlayerView,
        debug_interface: &PlayerDebugInterface<G>,
    ) -> Result<(), PlayerError>;
}

pub struct EmptyPlayer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyPlayerOptions;

impl<G: Game> Player<G> for EmptyPlayer
where
    G::Action: Default,
{
    fn get_action(
        &mut self,
        _: &G::PlayerView,
        _: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError> {
        Ok(default())
    }
    fn debug_update(
        &mut self,
        _: &G::PlayerView,
        _: &PlayerDebugInterface<G>,
    ) -> Result<(), PlayerError> {
        Ok(())
    }
}

impl<G: Game, T: Player<G> + ?Sized> Player<G> for Box<T> {
    fn get_action(
        &mut self,
        view: &G::PlayerView,
        debug_interface: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError> {
        (**self).get_action(view, debug_interface)
    }
    fn debug_update(
        &mut self,
        player_view: &G::PlayerView,
        debug_interface: &PlayerDebugInterface<G>,
    ) -> Result<(), PlayerError> {
        (**self).debug_update(player_view, debug_interface)
    }
}

pub struct ErroredPlayer(pub String);

impl<G: Game> Player<G> for ErroredPlayer {
    fn get_action(
        &mut self,
        _: &G::PlayerView,
        _: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError> {
        Err(PlayerError::IOError(std::io::Error::new(
            std::io::ErrorKind::Other,
            self.0.as_str(),
        )))
    }
    fn debug_update(
        &mut self,
        _: &G::PlayerView,
        _: &PlayerDebugInterface<G>,
    ) -> Result<(), PlayerError> {
        Err(PlayerError::IOError(std::io::Error::new(
            std::io::ErrorKind::Other,
            self.0.as_str(),
        )))
    }
}
