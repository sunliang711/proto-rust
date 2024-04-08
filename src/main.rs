use clap::Parser;

pub mod args;
pub mod compile;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    std::env::set_var("OUT_DIR", &args.output_dir);

    compile::compile(&args.input_file, &[])?;

    Ok(())
}
