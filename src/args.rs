#[derive(clap::Parser, Debug)]
#[command(version, about,long_about = None)]
pub struct Args {
    pub input_file: String,

    #[arg(short, long, default_value_t = String::from("."))]
    pub output_dir: String,

    #[arg(short, long, num_args = 0.., value_delimiter = ',')]
    pub includes: Vec<String>,
}

impl Args {
    pub fn print_info(&self) {
        if !self.includes.is_empty() {
            println!("includes: {:?}", self.includes);
        }

        println!("input file: {}", self.input_file);

        println!("output_dir: {}", self.output_dir);
    }
}
