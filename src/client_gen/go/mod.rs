use super::*;

pub type Generator = trans_gen::gens::go::Generator;

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "Go";
    const RUNNABLE: bool = true;
    type GenOptions = <Generator as trans_gen::Generator>::Options;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.add(&trans::Schema::of::<G::DebugState>());
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
            options.target_dir.join("debug_interface.go"),
            &project_file!(options, "debug_interface.go"),
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
