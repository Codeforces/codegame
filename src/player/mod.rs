use crate::*;

mod stream;
mod tcp;

pub use stream::*;
pub use tcp::*;

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error(display = "IO error: {}", 0)]
    IOError(#[error(cause)] std::io::Error),
}

impl From<std::io::Error> for PlayerError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

pub trait Player<G: Game>: Send {
    fn get_action(
        &mut self,
        view: &G::PlayerView,
        custom_data_handler: Option<&dyn Fn(G::CustomData)>,
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
        _: Option<&dyn Fn(G::CustomData)>,
    ) -> Result<G::Action, PlayerError> {
        Ok(default())
    }
}

impl<G: Game, T: Player<G> + ?Sized> Player<G> for Box<T> {
    fn get_action(
        &mut self,
        view: &G::PlayerView,
        custom_data_handler: Option<&dyn Fn(G::CustomData)>,
    ) -> Result<G::Action, PlayerError> {
        (**self).get_action(view, custom_data_handler)
    }
}

pub struct ErroredPlayer(pub String);

impl<G: Game> Player<G> for ErroredPlayer {
    fn get_action(
        &mut self,
        _: &G::PlayerView,
        _: Option<&dyn Fn(G::CustomData)>,
    ) -> Result<G::Action, PlayerError> {
        Err(PlayerError::IOError(std::io::Error::new(
            std::io::ErrorKind::Other,
            self.0.as_str(),
        )))
    }
}
