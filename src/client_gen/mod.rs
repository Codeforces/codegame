use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::*;

macro_rules! project_file {
    ($options: expr, $path:expr $(,)?) => {{
        use heck::*;
        include_str!($path)
            .replace("project_name", &$options.name.to_snake_case())
            .replace("ProjectName", &$options.name.to_camel_case())
            .as_str()
    }};
}

#[derive(Debug)]
pub struct Options<'a> {
    pub name: &'a str,
    pub target_dir: &'a Path,
    pub version: &'a str,
}

pub trait ClientGen<G: Game> {
    const NAME: &'static str;
    const RUNNABLE: bool;
    type GenOptions: Debug + Default;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()>;
    fn build_local(options: &Options) -> anyhow::Result<()>;
    fn run_local(options: &Options) -> anyhow::Result<Command>;
}

fn write_file<P: AsRef<Path>>(path: P, content: &str) -> anyhow::Result<()> {
    if let Some(dir) = path.as_ref().parent() {
        std::fs::create_dir_all(dir)?;
    }
    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}

macro_rules! all_langs {
    ($invoke:path) => {
        $invoke!(cpp);
        $invoke!(csharp);
        $invoke!(dlang);
        $invoke!(fsharp);
        $invoke!(go);
        $invoke!(java);
        $invoke!(javascript);
        $invoke!(kotlin);
        $invoke!(python);
        $invoke!(ruby);
        $invoke!(rust);
        $invoke!(scala);
        $invoke!(markdown);
    };
}

macro_rules! lang_mod {
    ($lang:ident) => {
        mod $lang;
    };
}
all_langs!(lang_mod);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TestMode {
    Gen,
    Build,
    Run,
    BuildDocker,
    RunDocker,
}

impl std::str::FromStr for TestMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "gen" => Self::Gen,
            "build" => Self::Build,
            "run" => Self::Run,
            "build-docker" => Self::BuildDocker,
            "run-docker" => Self::RunDocker,
            _ => anyhow::bail!("Only gen | build[-docker] | run[-docker]"),
        })
    }
}

#[derive(Debug)]
pub struct TestOptions {
    pub clean: bool,
    pub mode: TestMode,
    pub host_from_docker: Option<String>,
}

pub fn test<G, CG>(
    options: &Options,
    gen_options: CG::GenOptions,
    extra_files: &HashMap<&str, &str>,
    test_options: &TestOptions,
) -> anyhow::Result<()>
where
    G: Game,
    CG: ClientGen<G>,
    G::Options: Default,
    G::Action: Default,
    G::DebugState: Default,
{
    info!("Generating {} with options {:#?}", CG::NAME, gen_options);
    CG::gen(options, gen_options)?;
    for (path, contents) in extra_files {
        std::fs::write(options.target_dir.join(path), contents)?;
    }

    if CG::RUNNABLE && matches!(test_options.mode, TestMode::Build | TestMode::Run) {
        info!("Building...");
        CG::build_local(options)?;
    }

    let docker_image = format!("codegame-test:{}", CG::NAME);

    if CG::RUNNABLE
        && matches!(
            test_options.mode,
            TestMode::BuildDocker | TestMode::RunDocker
        )
    {
        info!("Building docker...");
        command("docker")
            .current_dir(options.target_dir)
            .arg("build")
            .arg("-t")
            .arg(&docker_image)
            .arg(".")
            .run()?;
        info!("Compiling base package...");
        let mut child = command("docker build -t codegame-test -")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        child.stdin.take().unwrap().write_all(
            format!(
                "FROM {}\nRUN mkdir /output && sh compile.sh base",
                docker_image
            )
            .as_bytes(),
        )?;
        let status = child.wait()?;
        if !status.success() {
            anyhow::bail!("Process exited with {}", status);
        }
    }

    if CG::RUNNABLE && matches!(test_options.mode, TestMode::Run | TestMode::RunDocker) {
        info!("Running...");
        const PORT: u16 = 31005;
        const TOKEN: &str = "CODEGAME_TOKEN";
        let mut command = match test_options.mode {
            TestMode::Run => {
                let mut command = CG::run_local(options)?;
                command.arg("127.0.0.1").arg(PORT.to_string()).arg(TOKEN);
                command
            }
            TestMode::RunDocker => {
                let mut command = command("docker");
                command
                    .arg("run")
                    .arg("--rm")
                    .arg("--name")
                    .arg("codegame-test")
                    .arg("--network")
                    .arg("host")
                    .arg("codegame-test")
                    .arg("sh")
                    .arg("run.sh");
                command
                    .arg(
                        test_options
                            .host_from_docker
                            .as_ref()
                            .map(|host| host.as_str())
                            .unwrap_or("localhost"),
                    )
                    .arg(PORT.to_string())
                    .arg(TOKEN);
                command.current_dir(options.target_dir);
                command
            }
            _ => unreachable!(),
        };
        let client_player = TcpPlayer::<G>::new(TcpPlayerOptions {
            host: if test_options.host_from_docker.is_some() {
                Some("0.0.0.0".to_owned())
            } else {
                None
            },
            port: PORT,
            accept_timeout: Some(10.0),
            timeout: Some(10.0),
            token: Some(TOKEN.to_owned()),
        });
        let client_thread = std::thread::spawn(move || {
            let start_time = std::time::Instant::now();
            command.run().expect("Running client failed");
            std::time::Instant::now().duration_since(start_time)
        });
        let players = vec![
            Box::new(futures::executor::block_on(client_player)?) as Box<_>,
            Box::new(EmptyPlayer) as Box<_>,
        ];
        let processor = GameProcessor::new(None, default(), players);
        processor.run(Some(&DebugInterface {
            debug_command_handler: Box::new(|_player_index, _global, _command| {}),
            debug_state: Box::new(|_player_index| default()),
        }));
        match client_thread.join() {
            Ok(duration) => info!("Client running time: {} ms", duration.as_millis()),
            Err(_e) => anyhow::bail!("Running client failed"),
        }
    }
    info!("Success");
    Ok(())
}

pub fn test_all<G>(
    options: &Options,
    extra_files: &HashMap<&str, HashMap<&str, &str>>,
    language_filter: Option<HashSet<&str>>,
    gen_options: &HashMap<String, serde_json::Value>,
    test_options: &TestOptions,
) -> anyhow::Result<()>
where
    G: Game,
    G::Options: Default,
    G::Action: Default,
    G::DebugState: Default,
{
    macro_rules! test {
        ($lang:ident) => {{
            type CG = trans_gen::GeneratorImpl<$lang::Generator>;
            if language_filter
                .as_ref()
                .map_or(true, |filter| filter.contains(<CG as ClientGen<G>>::NAME))
            {
                let empty_extra_files = HashMap::new();
                let language_target_dir = options.target_dir.join(<CG as ClientGen<G>>::NAME);
                test::<G, CG>(
                    &if language_filter
                        .as_ref()
                        .map_or(false, |filter| filter.len() == 1)
                    {
                        Options { ..*options }
                    } else {
                        Options {
                            target_dir: language_target_dir.as_ref(),
                            ..*options
                        }
                    },
                    gen_options
                        .get(<CG as ClientGen<G>>::NAME)
                        .map(|value| {
                            serde_json::from_value(value.clone())
                                .expect("Failed to parse gen options")
                        })
                        .unwrap_or_default(),
                    if let Some(extra_files) = extra_files.get(<CG as ClientGen<G>>::NAME) {
                        extra_files
                    } else {
                        &empty_extra_files
                    },
                    test_options,
                )?;
            }
        }};
    }
    if test_options.clean && options.target_dir.exists() {
        std::fs::remove_dir_all(options.target_dir)?;
    }
    all_langs!(test);
    Ok(())
}

trait CommandExt {
    fn run(&mut self) -> anyhow::Result<()>;
}

impl CommandExt for Command {
    fn run(&mut self) -> anyhow::Result<()> {
        let status = self.status()?;
        if !status.success() {
            anyhow::bail!("Process exited with {}", status);
        }
        Ok(())
    }
}

fn command(cmd: &str) -> Command {
    let mut parts = cmd.split_whitespace();
    let mut command = if cfg!(windows) {
        let mut command = Command::new("cmd");
        command.arg("/C").arg(parts.next().unwrap());
        command
    } else {
        Command::new(parts.next().unwrap())
    };
    for part in parts {
        command.arg(part);
    }
    command
}
