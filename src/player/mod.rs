use super::*;

mod stream;
mod tcp;

pub use stream::*;
pub use tcp::*;

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

pub trait Player<G: Game>: Send {
    fn get_action(
        &mut self,
        view: &G::PlayerView,
        debug_interface: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError>;
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
}

impl<G: Game, T: Player<G> + ?Sized> Player<G> for Box<T> {
    fn get_action(
        &mut self,
        view: &G::PlayerView,
        debug_interface: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError> {
        (**self).get_action(view, debug_interface)
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
}
