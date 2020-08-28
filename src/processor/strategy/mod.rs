use super::*;

mod repeat;
mod standard;

pub use repeat::*;
pub use standard::*;

pub trait GameProcessorStrategy<G: Game>: Send {
    fn process_turn(&mut self, actions: HashMap<usize, G::Action>) -> Vec<G::Event>;
    fn game(&self) -> &G;
    fn finished(&self) -> bool;
}
