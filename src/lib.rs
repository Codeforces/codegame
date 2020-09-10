pub mod prelude {
    pub use batbox::*;
    #[cfg(feature = "rendering")]
    pub use geng::{self, prelude::*};
}

use prelude::*;

#[cfg(feature = "rendering")]
mod app;
#[cfg(feature = "client-gen")]
pub mod client_gen;
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
pub enum GameInitOptions<G: Game> {
    Ready(#[serde(bound = "")] G),
    New(#[serde(bound = "")] G::Options),
}

impl<G: Game> Default for GameInitOptions<G>
where
    G::Options: Default,
{
    fn default() -> Self {
        Self::New(default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameInitConfig<G: Game> {
    LoadFrom(std::path::PathBuf),
    Create(G::OptionsPreset),
}

impl<G: Game> From<GameInitConfig<G>> for GameInitOptions<G> {
    fn from(config: GameInitConfig<G>) -> Self {
        match config {
            GameInitConfig::LoadFrom(path) => Self::Ready(
                Trans::read_from(&mut std::fs::read(path).expect("Failed to read file").as_slice())
                    .expect("Failed to parse file"),
            ),
            GameInitConfig::Create(preset) => Self::New(preset.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullOptions<G: Game> {
    pub seed: Option<u64>,
    #[serde(bound = "")]
    pub game: GameInitConfig<G>,
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
    type OptionsPreset: Debug
        + Serialize
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
    type ClientData: Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static;
    fn init(rng: &mut dyn RngCore, player_count: usize, options: Self::Options) -> Self;
    fn player_view(&self, player_index: usize) -> Self::PlayerView;
    fn process_turn(
        &mut self,
        rng: &mut dyn RngCore,
        actions: HashMap<usize, Self::Action>,
    ) -> Vec<Self::Event>;
    fn finished(&self) -> bool;
    fn results(&self) -> Self::Results;
}

#[derive(Serialize, Deserialize, Trans)]
#[trans(no_generics_in_name)]
pub enum ClientMessage<G: Game> {
    ClientDataMessage { data: G::ClientData },
    ActionMessage { action: G::Action },
}

#[derive(Serialize, Deserialize, Trans)]
#[trans(no_generics_in_name)]
pub struct ServerMessage<G: Game> {
    pub player_view: Option<G::PlayerView>,
}

#[cfg(feature = "rendering")]
pub trait RendererData<G: Game>: Diff {
    fn new(game: &G) -> Self;
    fn update(&mut self, events: &[G::Event], prev_game: &G, game: &G) {
        #![allow(unused_variables)]
        *self = Self::new(game);
    }
}

#[cfg(feature = "rendering")]
pub struct CurrentRenderState<'a, G: Game, T: RendererData<G>> {
    pub game: &'a G,
    pub renderer_data: &'a T,
    pub client_data: &'a HashMap<usize, Vec<G::ClientData>>,
}

#[cfg(feature = "rendering")]
pub struct RenderState<'a, G: Game, T: RendererData<G>> {
    pub current: CurrentRenderState<'a, G, T>,
    pub prev: Option<CurrentRenderState<'a, G, T>>,
    pub t: f64,
    pub prev_events: &'a [G::Event],
}

#[cfg(feature = "rendering")]
pub trait Renderer<G: Game>: 'static {
    type ExtraData: RendererData<G>;
    type Preferences: Debug + Clone + Default + Serialize + for<'de> Deserialize<'de> + 'static;
    fn default_tps(&self) -> f64;
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }
    fn draw(&mut self, state: RenderState<G, Self::ExtraData>, framebuffer: &mut ugli::Framebuffer);
    fn process_event(&mut self, event: &G::Event) {
        #![allow(unused_variables)]
    }
    fn handle_event(&mut self, event: &geng::Event) {
        #![allow(unused_variables)]
    }
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
