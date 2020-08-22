use super::*;

pub type Generator = trans_gen::JavaScript;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "JavaScript";
    fn gen(options: &Options) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version);
        gen.add(&trans_schema::schema::<ClientMessage<G>>());
        gen.add(&trans_schema::schema::<ServerMessage<G>>());
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
            options.target_dir.join("index.js"),
            project_file!(options, "index.js"),
        )?;
        write_file(
            options.target_dir.join("my-strategy.js"),
            project_file!(options, "my-strategy.js"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        Ok(())
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command("node");
        command.arg("index.js");
        command.current_dir(options.target_dir);
        Ok(command)
    }
}