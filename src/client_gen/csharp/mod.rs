use super::*;

pub type Generator = trans_gen::GeneratorImpl<trans_gen::gens::csharp::Generator>;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "CSharp";
    fn gen(options: &Options) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.add(&trans::Schema::of::<G::DebugData>());
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
            options.target_dir.join("Runner.cs"),
            &project_file!(options, "Runner.cs"),
        )?;
        write_file(
            options.target_dir.join("Debug.cs"),
            &project_file!(options, "Debug.cs"),
        )?;
        write_file(
            options.target_dir.join("MyStrategy.cs"),
            &project_file!(options, "MyStrategy.cs"),
        )?;
        write_file(
            options.target_dir.join(format!("{}.csproj", options.name)),
            &project_file!(options, "project.csproj"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        command("dotnet")
            .current_dir(options.target_dir)
            .arg("build")
            .arg("-c")
            .arg("Release")
            .run()
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command("dotnet");
        command.arg("run").arg("-c").arg("Release");
        command.current_dir(options.target_dir);
        Ok(command)
    }
}
