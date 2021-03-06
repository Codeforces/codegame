use super::*;

pub type Generator = trans_gen::gens::scala::Generator;

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "Scala";
    const RUNNABLE: bool = true;
    type GenOptions = <Generator as trans_gen::Generator>::Options;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options);
        let src_path = options.target_dir.join("src").join("main").join("scala");
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.add(&trans::Schema::of::<G::DebugState>());
        gen.result().write_to(&src_path)?;
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
            src_path.join("MyStrategy.scala"),
            &project_file!(options, "MyStrategy.scala"),
        )?;
        write_file(
            src_path.join("DebugInterface.scala"),
            &project_file!(options, "DebugInterface.scala"),
        )?;
        write_file(
            src_path.join("Runner.scala"),
            &project_file!(options, "Runner.scala"),
        )?;
        write_file(
            options.target_dir.join("pom.xml"),
            project_file!(options, "pom.xml"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        command("mvn")
            .current_dir(options.target_dir)
            .arg("package")
            .arg("--batch-mode")
            .run()
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command("java");
        command
            .arg("-jar")
            .arg(format!("target/{}-jar-with-dependencies.jar", options.name));
        command.current_dir(options.target_dir);
        Ok(command)
    }
}
