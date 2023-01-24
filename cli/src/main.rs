use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Zuhang Xu", about = "Find the word")]
struct Cli {
    word: String,
    //set the path to the file to read
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content =
        std::fs::read_to_string(&args.file).expect("Something went wrong reading the file");

    //return the line number that the word is found
    for (i, line) in content.lines().enumerate() {
        if line.contains(&args.word) {
            println!("{}: {}", i + 1, line);
        }
    }
}
