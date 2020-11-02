use super::*;

pub type Generator = trans_gen::GeneratorImpl<trans_gen::gens::markdown::Generator>;

impl<G: Game> ClientGen<G> for Generator {
    const NAME: &'static str = "Markdown";
    const RUNNABLE: bool = false;
    fn gen(options: &Options) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.result().write_to(options.target_dir)?;
        Ok(())
    }
    fn build_local(options: &Options) -> anyhow::Result<()> {
        unreachable!();
    }
    fn run_local(options: &Options) -> anyhow::Result<Command> {
        unreachable!();
    }
}
