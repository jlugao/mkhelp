use ansi_term::Colour;
use ansi_term::Style;
use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let header_re: Regex = Regex::new(r"^\#+@\s*(.+)$").unwrap();
    let item_re: Regex = Regex::new(r"^(\w+):\s*\#+\s*(.+)$").unwrap();
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        if header_re.is_match(line) {
            for cap in header_re.captures_iter(line) {
                println!("\n{}", Colour::Green.bold().paint(&cap[1]))
            }
        }
        if item_re.is_match(line) {
            for cap in item_re.captures_iter(line) {
                println!("- {}: {}", Style::new().bold().paint(&cap[1]), &cap[2])
            }
        }
    }
}
