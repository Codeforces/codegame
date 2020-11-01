use super::*;

pub type Generator = trans_gen::GeneratorImpl<trans_gen::gens::rust::Generator>;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "Rust";
    fn gen(options: &Options) -> anyhow::Result<()> {
        let mut gen = Self::new(&format!("{}-model", options.name), options.version);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.result().write_to(options.target_dir.join("model"))?;
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
            options.target_dir.join("Cargo.toml"),
            &project_file!(options, "Cargo.toml.template")
                .replace("$version", options.version)
                .replace("$trans-version", trans::VERSION),
        )?;
        write_file(
            options.target_dir.join("src/main.rs"),
            project_file!(options, "main.rs"),
        )?;
        write_file(
            options.target_dir.join("src/my_strategy.rs"),
            project_file!(options, "my_strategy.rs"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        command("cargo")
            .current_dir(options.target_dir)
            .arg("build")
            .arg("--release")
            .run()
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command("cargo");
        command.arg("run").arg("--release").arg("--");
        command.current_dir(options.target_dir);
        Ok(command)
    }
}
