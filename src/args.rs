#[derive(clap::Parser, Debug)]
#[command(version, about,long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input_file: String,

    #[arg(short, long, default_value_t = String::from("."))]
    pub output_dir: String,
}
