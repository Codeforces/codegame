use super::*;

impl<G: Game, R: Renderer<G>> History<G, R> {
    pub fn load(path: &str) -> impl Future<Output = Self> {
        fn load<G: Game, R: Renderer<G>>(
            path: &str,
        ) -> Result<impl Future<Output = History<G, R>>, Box<dyn std::error::Error>> {
            let xhr = stdweb::web::XmlHttpRequest::new();
            xhr.open("GET", path)?;
            xhr.set_response_type(stdweb::web::XhrResponseType::ArrayBuffer)?;
            xhr.send()?;
            let (sender, receiver) = futures::channel::oneshot::channel();
            use stdweb::web::IEventTarget;
            let loaded_handler = {
                let xhr = xhr.clone();
                let f = move || -> Result<(), Box<dyn std::error::Error>> {
                    let data: stdweb::web::ArrayBuffer =
                        stdweb::unstable::TryFrom::try_from(xhr.raw_response()).unwrap();
                    let data: Vec<u8> = data.into();
                    let mut reader = data.as_slice();
                    let initial_state = G::read_from(&mut reader)?;
                    let history = History::new(&initial_state);
                    let mut current_state = initial_state;
                    let mut tick_handler = history.tick_handler();
                    while !reader.fill_buf()?.is_empty() {
                        let events = Vec::<G::Event>::read_from(&mut reader)?;
                        let delta = G::Delta::read_from(&mut reader)?;
                        current_state.update(&delta);
                        tick_handler(&current_state, events);
                    }
                    let _ = sender.send(history);
                    Ok(())
                };
                move || {
                    f().expect("Error while reading replay");
                }
            };
            xhr.add_event_listener({
                let xhr = xhr.clone();
                let mut loaded_handler = Some(loaded_handler);
                move |event: stdweb::web::event::ProgressLoadEvent| {
                    loaded_handler.take().unwrap()();
                }
            });
            Ok(receiver.map(|result| result.expect("Failed to load replay")))
        }
        load::<G, R>(path).expect("Failed to load replay")
    }
}
