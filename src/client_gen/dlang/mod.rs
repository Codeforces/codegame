use super::*;

pub type Generator = trans_gen::gens::dlang::Generator;

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "Dlang";
    const RUNNABLE: bool = true;
    type GenOptions = <Generator as trans_gen::Generator>::Options;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
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
            options.target_dir.join("source").join("debugger.d"),
            &project_file!(options, "debugger.d"),
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
