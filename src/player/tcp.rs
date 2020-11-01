use super::*;

pub struct TcpPlayer<G: Game> {
    inner: StreamPlayer<G>,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TcpPlayerOptions {
    pub host: Option<String>,
    pub port: u16,
    pub accept_timeout: Option<f64>,
    pub timeout: Option<f64>,
    pub token: Option<String>,
}

impl<G: Game> TcpPlayer<G> {
    pub fn new(options: TcpPlayerOptions) -> impl Future<Output = Result<Self, std::io::Error>> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        std::thread::spawn(move || {
            let result = {
                let sender = &sender;
                let f = move || -> Result<Self, std::io::Error> {
                    let listener = std::net::TcpListener::bind((
                        options
                            .host
                            .as_ref()
                            .map(|host| host.as_str())
                            .unwrap_or("127.0.0.1"),
                        options.port,
                    ))?;
                    listener.set_nonblocking(true)?;
                    info!("Waiting for connection on port {}", options.port);
                    let timer = Timer::new();
                    while !sender.is_canceled() {
                        if let Some(time) = options.accept_timeout {
                            if timer.elapsed() > time {
                                info!("Timeout accepting player on port {}", options.port);
                                break;
                            }
                        }
                        match listener.accept() {
                            Ok((stream, _)) => {
                                info!("Got connection on port {}", options.port);
                                if let Some(time) = options.timeout {
                                    stream.set_read_timeout(Some(
                                        std::time::Duration::from_millis((time * 1000.0) as _),
                                    ))?;
                                    stream.set_write_timeout(Some(
                                        std::time::Duration::from_millis((time * 1000.0) as _),
                                    ))?;
                                }
                                stream.set_nonblocking(false)?;
                                stream.set_nodelay(true)?;
                                let stream_clone = stream.try_clone()?;
                                let mut reader = std::io::BufReader::new(stream);
                                let writer = std::io::BufWriter::new(stream_clone);
                                let token: String = Trans::read_from(&mut reader)?;
                                if let Some(actual_token) = &options.token {
                                    if token != actual_token.as_str() {
                                        return Err(std::io::Error::new(
                                            std::io::ErrorKind::Other,
                                            "Token mismatch",
                                        ));
                                    }
                                }
                                return Ok(Self {
                                    inner: StreamPlayer::new(Box::new(reader), Box::new(writer)),
                                    port: options.port,
                                });
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                std::thread::sleep(std::time::Duration::from_millis(100));
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    info!("Stop listening port {}", options.port);
                    return Err(std::io::Error::from(std::io::ErrorKind::ConnectionReset));
                };
                f()
            };
            let _ = sender.send(result);
        });
        receiver.map(|result| result.unwrap())
    }
}

impl<G: Game> Drop for TcpPlayer<G> {
    fn drop(&mut self) {
        info!("Dropping tcp player on port {}", self.port);
    }
}

impl<G: Game> Player<G> for TcpPlayer<G> {
    fn get_action(
        &mut self,
        player_view: &G::PlayerView,
        debug_interface: Option<&PlayerDebugInterface<G>>,
    ) -> Result<G::Action, PlayerError> {
        Player::<G>::get_action(&mut self.inner, player_view, debug_interface)
    }
    fn debug_update(
        &mut self,
        debug_interface: &PlayerDebugInterface<G>,
    ) -> Result<(), PlayerError> {
        Player::<G>::debug_update(&mut self.inner, debug_interface)
    }
}
