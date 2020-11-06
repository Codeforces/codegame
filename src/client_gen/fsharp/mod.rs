use super::*;

pub type Generator = trans_gen::gens::fsharp::Generator;

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "FSharp";
    const RUNNABLE: bool = true;
    type GenOptions = <Generator as trans_gen::Generator>::Options;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.add(&trans::Schema::of::<G::DebugState>());
        let result = gen.result();
        result.write_to(options.target_dir)?;
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
            options.target_dir.join("DebugInterface.fs"),
            &project_file!(options, "DebugInterface.fs"),
        )?;
        write_file(
            options.target_dir.join("Runner.fs"),
            &project_file!(options, "Runner.fs"),
        )?;
        write_file(
            options.target_dir.join("MyStrategy.fs"),
            &project_file!(options, "MyStrategy.fs"),
        )?;
        write_file(
            options.target_dir.join(format!("{}.fsproj", options.name)),
            &project_file!(options, "project.fsproj").replace("<SourceFile />", {
                result
                    .files
                    .iter()
                    .filter_map(|file| {
                        if file.path.ends_with(".fs") {
                            Some(file.path.as_str())
                        } else {
                            None
                        }
                    })
                    .chain(std::iter::once("DebugInterface.fs"))
                    .chain(std::iter::once("MyStrategy.fs"))
                    .chain(std::iter::once("Runner.fs"))
                    .map(|path| format!("<Compile Include=\"{}\" />", path))
                    .collect::<Vec<_>>()
                    .join("\n        ")
                    .as_str()
            }),
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
