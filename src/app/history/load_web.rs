use super::*;

impl<G: Game, T: RendererData<G>> History<G, T> {
    pub fn load(path: &str) -> impl Future<Output = Self> {
        fn load<G: Game, T: RendererData<G>>(
            path: &str,
        ) -> Result<impl Future<Output = History<G, T>>, Box<dyn std::error::Error>> {
            let xhr = web_sys::XmlHttpRequest::new().unwrap();
            xhr.open("GET", path).unwrap();
            xhr.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
            xhr.send().unwrap();
            let (sender, receiver) = futures::channel::oneshot::channel();
            let loaded_handler = {
                let xhr = xhr.clone();
                let f = move || -> Result<(), Box<dyn std::error::Error>> {
                    let data = js_sys::Uint8Array::new(
                        xhr.response()
                            .unwrap()
                            .dyn_into::<js_sys::ArrayBuffer>()
                            .unwrap()
                            .as_ref(),
                    )
                    .to_vec();
                    let mut reader = data.as_slice();
                    let initial_state = G::read_from(&mut reader)?;
                    let history = History::new(initial_state.clone());
                    let mut current_state = initial_state;
                    let mut tick_handler = history.tick_handler();
                    while !BufRead::fill_buf(&mut reader)?.is_empty() {
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
            let handler = {
                let mut loaded_handler = Some(loaded_handler);
                move |_: web_sys::ProgressEvent| {
                    loaded_handler.take().unwrap()();
                }
            };
            let handler = wasm_bindgen::closure::Closure::wrap(
                Box::new(handler) as Box<dyn FnMut(web_sys::ProgressEvent)>
            );
            xhr.add_event_listener_with_callback("load", handler.as_ref().unchecked_ref())
                .unwrap();
            handler.forget(); // TODO: not forget
            Ok(receiver.map(|result| result.expect("Failed to load replay")))
        }
        load::<G, T>(path).expect("Failed to load replay")
    }
}
