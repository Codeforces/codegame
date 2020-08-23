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
    fn gen(options: &Options) -> anyhow::Result<()>;
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
}

impl std::str::FromStr for TestMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "gen" => Self::Gen,
            "build" => Self::Build,
            "run" => Self::Run,
            _ => anyhow::bail!("Only gen | build | run"),
        })
    }
}

#[derive(Debug)]
pub struct TestOptions {
    pub clean: bool,
    pub mode: TestMode,
}

pub fn test<G, CG>(
    options: &Options,
    extra_files: &HashMap<&str, &str>,
    test_options: &TestOptions,
) -> anyhow::Result<()>
where
    G: Game,
    CG: ClientGen<G>,
    G::Options: Default,
{
    info!("Generating {}", CG::NAME);
    CG::gen(options)?;
    for (path, contents) in extra_files {
        std::fs::write(options.target_dir.join(path), contents)?;
    }

    if matches!(test_options.mode, TestMode::Build | TestMode::Run) {
        info!("Building...");
        CG::build_local(options)?;
    }

    if matches!(test_options.mode, TestMode::Run) {
        info!("Running...");
        const PORT: u16 = 31005;
        const TOKEN: &str = "CODEGAME_TOKEN";
        let mut command = CG::run_local(options)?;
        let client_thread = std::thread::spawn(move || {
            command.arg("127.0.0.1").arg(PORT.to_string()).arg(TOKEN);
            command.run().expect("Running client failed");
        });
        let players = vec![Box::new(futures::executor::block_on(TcpPlayer::<G>::new(
            TcpPlayerOptions {
                host: None,
                port: PORT,
                accept_timeout: Some(10.0),
                timeout: Some(10.0),
                token: Some(TOKEN.to_owned()),
            },
        ))?) as Box<_>];
        let processor = GameProcessor::new(None, default(), players);
        processor.run();
        if let Err(e) = client_thread.join() {
            anyhow::bail!("Running client failed");
        }
    }
    info!("Success");
    Ok(())
}

pub fn test_all<G>(
    options: &Options,
    extra_files: &HashMap<&str, HashMap<&str, &str>>,
    language_filter: Option<HashSet<&str>>,
    test_options: &TestOptions,
) -> anyhow::Result<()>
where
    G: Game,
    G::Options: Default,
{
    macro_rules! test {
        ($lang:ident) => {{
            type CG = $lang::Generator;
            if language_filter
                .as_ref()
                .map_or(true, |filter| filter.contains(<CG as ClientGen<G>>::NAME))
            {
                let empty_extra_files = HashMap::new();
                test::<G, CG>(
                    &Options {
                        target_dir: options.target_dir.join(<CG as ClientGen<G>>::NAME).as_ref(),
                        ..*options
                    },
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
