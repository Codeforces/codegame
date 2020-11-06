use super::*;

pub type Generator = trans_gen::gens::markdown::Generator;

impl<G: Game> ClientGen<G> for trans_gen::GeneratorImpl<Generator> {
    const NAME: &'static str = "Markdown";
    const RUNNABLE: bool = false;
    type GenOptions = <Generator as trans_gen::Generator>::Options;
    fn gen(options: &Options, gen_options: Self::GenOptions) -> anyhow::Result<()> {
        let mut gen = Self::new(options.name, options.version, gen_options);
        gen.add(&trans::Schema::of::<ClientMessage<G>>());
        gen.add(&trans::Schema::of::<ServerMessage<G>>());
        gen.add(&trans::Schema::of::<G::DebugState>());
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
