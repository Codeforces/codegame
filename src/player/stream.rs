use super::*;

struct Stream {
    reader: Box<dyn BufRead + Send>,
    writer: Box<dyn Write + Send>,
}

pub struct StreamPlayer<G: Game> {
    stream: Option<Stream>,
    phantom_data: PhantomData<G>,
}

impl<G: Game> StreamPlayer<G> {
    pub fn new(reader: Box<dyn BufRead + Send>, writer: Box<dyn Write + Send>) -> Self {
        Self {
            stream: Some(Stream { reader, writer }),
            phantom_data: PhantomData,
        }
    }
}

impl<G: Game> Drop for StreamPlayer<G> {
    fn drop(&mut self) {
        if let Some(stream) = &mut self.stream {
            let mut try_write_finish = || {
                ServerMessage::<G> { player_view: None }.write_to(&mut stream.writer)?;
                stream.writer.flush()
            };
            if let Err(e) = try_write_finish() {
                warn!("{}", e);
            }
        }
    }
}

impl<G: Game> Player<G> for StreamPlayer<G> {
    fn get_action(
        &mut self,
        player_view: &G::PlayerView,
        debug_interface: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError> {
        let stream = self.stream.as_mut().expect("Called get_action after error");
        let mut get_action = move || {
            ServerMessage::<G> {
                player_view: Some(player_view.clone()), // TODO: dont clone
            }
            .write_to(&mut stream.writer)?;
            stream.writer.flush()?;
            loop {
                match ClientMessage::<G>::read_from(&mut stream.reader)? {
                    ClientMessage::ActionMessage { action } => return Ok(action),
                    ClientMessage::DebugDataMessage { data } => {
                        if let Some(debug_interface) = debug_interface {
                            debug_interface.send(data);
                        }
                    }
                }
            }
        };
        let result = get_action();
        if result.is_err() {
            self.stream = None;
        }
        result
    }
}
