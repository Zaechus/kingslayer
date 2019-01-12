fn main() {
    let cli = kingslayer::get_world("data/world.json");

    println!("{}", cli.ask("l"));
    loop {
        match cli.ask(&cli.prompt()).as_str() {
            ref s => {
                println!("{}", s);
                if s.contains("You died.") {
                    break;
                }
            }
        }
    }
}
