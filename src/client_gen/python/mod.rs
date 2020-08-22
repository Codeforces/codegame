use super::*;

pub type Generator = trans_gen::GeneratorImpl<trans_gen::gens::python::Generator>;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "Python";
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
            options.target_dir.join("main.py"),
            project_file!(options, "main.py"),
        )?;
        write_file(
            options.target_dir.join("my_strategy.py"),
            project_file!(options, "my_strategy.py"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        Ok(())
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command(if cfg!(windows) { "py -3" } else { "python3" });
        command.arg("main.py");
        command.current_dir(options.target_dir);
        Ok(command)
    }
}
