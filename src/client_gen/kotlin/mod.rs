use super::*;

pub type Generator = trans_gen::gens::kotlin::Generator;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Compiler {
    Vanilla,
    GraalVM,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::Vanilla
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GenOptions {
    #[serde(default)]
    compiler: Compiler,
    #[serde(default)]
    trans: <Generator as trans_gen::Generator>::Options,
}

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "Kotlin";
    const RUNNABLE: bool = true;
    type GenOptions = GenOptions;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options.trans);
        let src_path = options.target_dir.join("src").join("main").join("kotlin");
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.add(&trans::Schema::of::<G::DebugState>());
        gen.result().write_to(&src_path)?;
        write_file(
            src_path.join("MyStrategy.kt"),
            &project_file!(options, "MyStrategy.kt"),
        )?;
        write_file(
            src_path.join("DebugInterface.kt"),
            &project_file!(options, "DebugInterface.kt"),
        )?;
        write_file(
            src_path.join("Runner.kt"),
            &project_file!(options, "Runner.kt"),
        )?;
        match gen_options.compiler {
            Compiler::Vanilla => {
                write_file(
                    options.target_dir.join("Dockerfile"),
                    project_file!(options, "vanilla/Dockerfile"),
                )?;
                write_file(
                    options.target_dir.join("compile.sh"),
                    project_file!(options, "vanilla/compile.sh"),
                )?;
                write_file(
                    options.target_dir.join("run.sh"),
                    project_file!(options, "vanilla/run.sh"),
                )?;
                write_file(
                    options.target_dir.join("pom.xml"),
                    project_file!(options, "vanilla/pom.xml"),
                )?;
            }
            Compiler::GraalVM => {
                write_file(
                    options.target_dir.join("Dockerfile"),
                    project_file!(options, "graalvm/Dockerfile"),
                )?;
                write_file(
                    options.target_dir.join("compile.sh"),
                    project_file!(options, "graalvm/compile.sh"),
                )?;
                write_file(
                    options.target_dir.join("run.sh"),
                    project_file!(options, "graalvm/run.sh"),
                )?;
                write_file(
                    options.target_dir.join("pom.xml"),
                    project_file!(options, "graalvm/pom.xml"),
                )?;
            }
        }
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
