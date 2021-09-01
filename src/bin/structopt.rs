use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CommandLine {
    #[structopt(short, long, default_value = "pattern")]
    pattern: String,

    #[structopt(short = "x", long, default_value="abc")]
    path: std::path::PathBuf,
}

fn main() {
    let p = CommandLine::from_args();

    println!("{:#?}", p)
}
