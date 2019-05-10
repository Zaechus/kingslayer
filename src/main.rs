use kingslayer::Cli;

fn main() {
    let cli = Cli::from_json_file("data/zork.json");

    cli.start();
}
