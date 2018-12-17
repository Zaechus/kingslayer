fn main() {
    let cli = kingslayer::get_world("data/world.json");

    println!("{}", cli.ask("l"));
    loop {
        println!("{}", cli.ask(&cli.prompt()));
    }
}
