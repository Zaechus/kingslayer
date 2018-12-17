fn main() {
    let cli = kingslayer::get_world("data/world.json");

    loop {
        println!("{}", cli.ask(&cli.prompt()));
    }
}
