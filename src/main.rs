use kingslayer::Cli;

fn main() {
    let cli = Cli::from_ron_file("data/world.ron");

    cli.start();
}
