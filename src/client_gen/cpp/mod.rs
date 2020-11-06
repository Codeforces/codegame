use super::*;

pub type Generator = trans_gen::gens::cpp::Generator;

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "Cpp";
    const RUNNABLE: bool = true;
    type GenOptions = <Generator as trans_gen::Generator>::Options;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options);
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
            options.target_dir.join("CMakeLists.txt"),
            &project_file!(options, "CMakeLists.txt"),
        )?;
        write_file(
            options.target_dir.join("TcpStream.hpp"),
            &project_file!(options, "TcpStream.hpp"),
        )?;
        write_file(
            options.target_dir.join("TcpStream.cpp"),
            &project_file!(options, "TcpStream.cpp"),
        )?;
        write_file(
            options.target_dir.join("DebugInterface.hpp"),
            &project_file!(options, "DebugInterface.hpp"),
        )?;
        write_file(
            options.target_dir.join("DebugInterface.cpp"),
            &project_file!(options, "DebugInterface.cpp"),
        )?;
        write_file(
            options.target_dir.join("MyStrategy.hpp"),
            &project_file!(options, "MyStrategy.hpp"),
        )?;
        write_file(
            options.target_dir.join("MyStrategy.cpp"),
            &project_file!(options, "MyStrategy.cpp"),
        )?;
        write_file(
            options.target_dir.join("main.cpp"),
            &project_file!(options, "main.cpp"),
        )?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        let standard: &str = "17";
        command("cmake")
            .current_dir(options.target_dir)
            .arg(format!("-DCMAKE_CXX_STANDARD={}", standard))
            .arg("-DCMAKE_BUILD_TYPE=RELEASE")
            .arg("-DCMAKE_VERBOSE_MAKEFILE=ON")
            .arg(".")
            .run()?;
        command("cmake")
            .current_dir(options.target_dir)
            .arg("--build")
            .arg(".")
            .arg("--config")
            .arg("Release")
            .run()?;
        Ok(())
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        let mut command = command(
            PathBuf::from(if cfg!(windows) { "Release" } else { "." })
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
