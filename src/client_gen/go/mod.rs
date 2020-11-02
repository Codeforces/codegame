use super::*;

pub type Generator = trans_gen::GeneratorImpl<trans_gen::gens::go::Generator>;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "Go";
    const RUNNABLE: bool = true;
    fn gen(options: &Options) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.result().write_to(options.target_dir)?;
        write_file(
            options.target_dir.join("Dockerfile"),
            project_file!(options, "Dockerfile"),
        )?;
        write_file(
            options.target_dir.join("compile.sh"),
            project_file!(options, "compile.sh"),
        )?;
        write_file(
            options.target_dir.join("run.sh"),
            project_file!(options, "run.sh"),
        )?;
        write_file(
            options.target_dir.join("go.mod"),
            &project_file!(options, "go.mod"),
        )?;
        write_file(
            options.target_dir.join("debug.go"),
            &project_file!(options, "debug.go"),
        )?;
        write_file(
            options.target_dir.join("main.go"),
            &project_file!(options, "main.go"),
        )?;
        write_file(
            options.target_dir.join("my_strategy.go"),
            &project_file!(options, "my_strategy.go"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        command("go")
            .current_dir(options.target_dir)
            .arg("build")
            .arg("-o")
            .arg(format!(
                "{}{}",
                options.name,
                if cfg!(windows) { ".exe" } else { "" }
            ))
            .run()
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command(
            PathBuf::from(".")
                .join(format!(
                    "{}{}",
                    options.name,
                    if cfg!(windows) { ".exe" } else { "" }
                ))
                .to_str()
                .unwrap(),
        );
        command.current_dir(options.target_dir);
        Ok(command)
    }
}
