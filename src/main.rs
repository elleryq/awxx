use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    command: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
