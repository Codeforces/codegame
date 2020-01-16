pub mod prelude {
    pub use batbox::*;
    #[cfg(feature = "rendering")]
    pub use geng::{self, prelude::*};
}

use prelude::*;

#[cfg(feature = "rendering")]
mod app;
mod player;
mod processor;

#[cfg(feature = "rendering")]
pub use app::*;
pub use player::*;
pub use processor::*;

pub trait PlayerOptions<G: Game>: From<TcpPlayerOptions> + From<EmptyPlayerOptions> {
    fn get(
        &self,
        extra_data: &G::PlayerExtraData,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Player<G>>, PlayerError>>>>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullOptions<G: Game> {
    pub seed: Option<u64>,
    pub options_preset: G::OptionsPreset,
    pub players: Vec<G::PlayerOptions>,
}

impl<G: Game> FullOptions<G> {
    pub fn save(&self, writer: impl Write) -> std::io::Result<()> {
        Ok(serde_json::to_writer_pretty(writer, self)?)
    }
    pub fn load(reader: impl Read) -> std::io::Result<Self> {
        Ok(serde_json::from_reader(reader)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullResults<G: Game> {
    players: Vec<PlayerResult>,
    results: G::Results,
    seed: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerResult {
    crashed: bool,
    comment: Option<String>,
}

pub trait Game: Diff {
    type Options: Serialize + for<'de> Deserialize<'de> + Sync + Send + Clone + 'static;
    type OptionsPreset: Serialize
        + for<'de> Deserialize<'de>
        + Sync
        + Send
        + Clone
        + 'static
        + Into<Self::Options>;
    type PlayerOptions: PlayerOptions<Self>
        + Serialize
        + for<'de> Deserialize<'de>
        + Sync
        + Send
        + Clone
        + 'static;
    type Action: Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static;
    type Event: Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static;
    type PlayerView: Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static;
    type Results: Serialize + for<'de> Deserialize<'de> + Sync + Send + Clone + 'static;
    type PlayerExtraData;
    type CustomData: Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static;
    fn init(rng: &mut dyn rand::RngCore, player_count: usize, options: Self::Options) -> Self;
    fn player_view(&self, player_index: usize) -> Self::PlayerView;
    fn process_turn(
        &mut self,
        rng: &mut dyn rand::RngCore,
        actions: HashMap<usize, Self::Action>,
    ) -> Vec<Self::Event>;
    fn finished(&self) -> bool;
    fn results(&self) -> Self::Results;
}

#[derive(Serialize, Deserialize, Trans, Schematic)]
pub enum PlayerMessage<G: Game> {
    CustomDataMessage { data: G::CustomData },
    ActionMessage { action: G::Action },
}

#[derive(Serialize, Deserialize, Trans, Schematic)]
pub struct ServerMessage<G: Game> {
    pub player_view: Option<G::PlayerView>,
}

#[cfg(feature = "rendering")]
pub trait RendererExtraData<G: Game>: Diff {
    fn new(game: &G) -> Self;
    fn update(&mut self, events: &[G::Event], game: &G) {
        *self = Self::new(game);
    }
}

#[cfg(feature = "rendering")]
pub trait Renderer<G: Game>: 'static {
    type ExtraData: RendererExtraData<G>;
    type Preferences: Debug + Clone + Default + Serialize + for<'de> Deserialize<'de> + 'static;
    fn default_tps(&self) -> f64;
    fn update(&mut self, delta_time: f64) {}
    fn draw(
        &mut self,
        game: &G,
        extra_data: &Self::ExtraData,
        custom_data: &HashMap<usize, Vec<G::CustomData>>,
        framebuffer: &mut ugli::Framebuffer,
    );
    fn process_event(&mut self, event: &G::Event) {}
    fn handle_event(&mut self, event: &geng::Event) {}
}

pub fn save_replay_tick_handler<G: Game, T: Write + Send + 'static>(
    mut writer: T,
) -> Box<dyn FnMut(Option<&Vec<G::Event>>, &G) + Send> {
    let mut last: Option<G> = None;
    Box::new(move |events: Option<&Vec<G::Event>>, current: &G| {
        if let Some(events) = events {
            events
                .write_to(&mut writer)
                .expect("Failed to write replay");
        }
        match &last {
            None => current.write_to(&mut writer),
            Some(last) => last.diff(current).write_to(&mut writer),
        }
        .expect("Failed to write replay");
        last = Some(current.clone());
    })
}
