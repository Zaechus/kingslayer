use kingslayer::Cli;

fn main() {
    let cli = Cli::from_json_file("data/world.json");

    cli.start();
}
