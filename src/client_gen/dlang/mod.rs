use super::*;

pub type Generator = trans_gen::Dlang;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "Dlang";
    fn gen(options: &Options) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version);
        gen.add(&trans_schema::schema::<ClientMessage<G>>());
        gen.add(&trans_schema::schema::<ServerMessage<G>>());
        gen.result().write_to(options.target_dir.join("source"))?;
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
            options.target_dir.join("dub.json"),
            &project_file!(options, "dub.json"),
        )?;
        write_file(
            options.target_dir.join("source").join("app.d"),
            &project_file!(options, "app.d"),
        )?;
        write_file(
            options.target_dir.join("source").join("my_strategy.d"),
            &project_file!(options, "my_strategy.d"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        command("dub")
            .current_dir(options.target_dir)
            .arg("build")
            .arg("-b")
            .arg("release")
            .run()
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command("dub");
        command.arg("run").arg("-b").arg("release").arg("--");
        command.current_dir(options.target_dir);
        Ok(command)
    }
}
