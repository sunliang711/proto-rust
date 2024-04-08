use clap::Parser;

pub mod args;
pub mod compile;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    args.print_info();

    std::env::set_var("OUT_DIR", &args.output_dir);
    let includes: Vec<&str> = args.includes.iter().map(|x| x.as_str()).collect();

    compile::compile(&args.input_file, &includes)?;

    Ok(())
}
